use std::ops::Range;

use async_tiff::decoder::DecoderRegistry;
use async_tiff::error::AsyncTiffResult;
use async_tiff::metadata::TiffMetadataReader;
use async_tiff::reader::AsyncFileReader;
use async_tiff::tags::PlanarConfiguration;
use async_tiff::TypedArray;

use crate::{Error, Raster, RasterFormat};

#[derive(Debug)]
struct BytesReader(bytes::Bytes);

#[async_trait::async_trait]
impl AsyncFileReader for BytesReader {
    async fn get_bytes(&self, range: Range<u64>) -> AsyncTiffResult<bytes::Bytes> {
        Ok(self.0.slice(range.start as usize..range.end as usize))
    }
}

pub struct GeoTiffSource {
    bytes: bytes::Bytes,
}

impl GeoTiffSource {
    pub fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoTiffSource { bytes }
    }

    pub async fn load(self) -> Result<Raster, Error> {
        let reader = BytesReader(self.bytes);

        let mut metadata_reader = TiffMetadataReader::try_open(&reader).await?;
        let tiff = metadata_reader.read(&reader).await?;

        let ifd = tiff.ifds().first().ok_or(Error::MissingGeoInfo)?;

        let width = ifd.image_width();
        let height = ifd.image_height();

        // Read geo-referencing tags: try tiepoint+scale first, then ModelTransformationTag
        let (origin_x, origin_y, scale_x, scale_y) =
            if let (Some(tiepoint), Some(scale)) =
                (ifd.model_tiepoint(), ifd.model_pixel_scale())
            {
                let origin_x = *tiepoint.get(3).ok_or(Error::MissingGeoInfo)?;
                let origin_y = *tiepoint.get(4).ok_or(Error::MissingGeoInfo)?;
                let scale_x = *scale.first().ok_or(Error::MissingGeoInfo)?;
                let scale_y = *scale.get(1).ok_or(Error::MissingGeoInfo)?;
                (origin_x, origin_y, scale_x, scale_y)
            } else if let Some(matrix) = ifd.model_transformation() {
                let scale_x = *matrix.first().ok_or(Error::MissingGeoInfo)?;
                // Negate matrix[5] to match the positive convention used by ModelPixelScaleTag
                let scale_y = -(*matrix.get(5).ok_or(Error::MissingGeoInfo)?);
                let origin_x = *matrix.get(3).ok_or(Error::MissingGeoInfo)?;
                let origin_y = *matrix.get(7).ok_or(Error::MissingGeoInfo)?;
                (origin_x, origin_y, scale_x, scale_y)
            } else {
                return Err(Error::MissingGeoInfo);
            };

        let min_x = origin_x;
        let max_y = origin_y;
        let max_x = origin_x + scale_x * f64::from(width);
        let min_y = origin_y - scale_y * f64::from(height);

        let extent = geo_types::Rect::new(
            geo_types::coord! { x: min_x, y: min_y },
            geo_types::coord! { x: max_x, y: max_y },
        );

        let tile_width = ifd.tile_width().ok_or(Error::UnsupportedColorType)?;
        let tile_height = ifd.tile_height().ok_or(Error::UnsupportedColorType)?;
        let (tiles_x, tiles_y) = ifd.tile_count().ok_or(Error::UnsupportedColorType)?;

        let samples_per_pixel = ifd.samples_per_pixel();
        let decoder_registry = DecoderRegistry::default();
        let is_planar = ifd.planar_configuration() == PlanarConfiguration::Planar;

        // Parse nodata value from GDAL metadata
        let nodata_f64: Option<f64> = ifd
            .gdal_nodata()
            .and_then(|s| s.trim().parse::<f64>().ok());

        // Fetch, decode, and normalize all tiles to chunky layout
        let mut decoded_tiles: Vec<(usize, usize, TypedArray, [usize; 3])> =
            Vec::with_capacity(tiles_x * tiles_y);
        for ty in 0..tiles_y {
            for tx in 0..tiles_x {
                let tile = ifd.fetch_tile(tx, ty, &reader).await?;
                let array = tile.decode(&decoder_registry)?;
                let (data, shape, _) = array.into_inner();

                let (data, shape) = if is_planar {
                    let [bands, h, w] = shape;
                    (planar_to_chunky(data, bands, h, w), [h, w, bands])
                } else {
                    (data, shape)
                };

                decoded_tiles.push((tx, ty, data, shape));
            }
        }

        // Determine pixel format from first tile and stitch
        let first_data = &decoded_tiles
            .first()
            .ok_or(Error::UnsupportedColorType)?
            .2;

        let (data, format) = match (samples_per_pixel, first_data) {
            (1, TypedArray::UInt8(_)) => {
                let mut buf = vec![0u8; (width * height) as usize];
                stitch_tiles_u8(&decoded_tiles, &mut buf, width, height, tile_width, tile_height, 1);
                let nodata_u8 = nodata_f64.map(|v| v as u8);
                let (min_val, max_val) = min_max_u8(&buf, nodata_u8);
                if nodata_u8.is_some() {
                    let nd = nodata_u8.unwrap();
                    let mut rgba = Vec::with_capacity(buf.len() * 4);
                    for &v in &buf {
                        if v == nd {
                            rgba.extend_from_slice(&[0, 0, 0, 0]);
                        } else {
                            let n = normalize_u8(v, min_val, max_val);
                            rgba.extend_from_slice(&[n, n, n, 255]);
                        }
                    }
                    (rgba, RasterFormat::Rgba8)
                } else {
                    let data: Vec<u8> = buf.iter().map(|&v| normalize_u8(v, min_val, max_val)).collect();
                    (data, RasterFormat::R8)
                }
            }
            (1, TypedArray::UInt16(_)) => {
                let mut buf16 = vec![0u16; (width * height) as usize];
                stitch_tiles_u16(
                    &decoded_tiles,
                    &mut buf16,
                    width,
                    height,
                    tile_width,
                    tile_height,
                    1,
                );
                let nodata_u16 = nodata_f64.map(|v| v as u16);
                let (min_val, max_val) = min_max_u16(&buf16, nodata_u16);
                if nodata_u16.is_some() {
                    let nd = nodata_u16.unwrap();
                    let mut rgba = Vec::with_capacity(buf16.len() * 4);
                    for &v in &buf16 {
                        if v == nd {
                            rgba.extend_from_slice(&[0, 0, 0, 0]);
                        } else {
                            let n = normalize_u16(v, min_val, max_val);
                            rgba.extend_from_slice(&[n, n, n, 255]);
                        }
                    }
                    (rgba, RasterFormat::Rgba8)
                } else {
                    let data: Vec<u8> = buf16.iter().map(|&v| normalize_u16(v, min_val, max_val)).collect();
                    (data, RasterFormat::R8)
                }
            }
            (3, TypedArray::UInt8(_)) => {
                let mut buf = vec![0u8; (width * height) as usize * 3];
                stitch_tiles_u8(&decoded_tiles, &mut buf, width, height, tile_width, tile_height, 3);
                let pixel_count = (width * height) as usize;
                let nodata_u8 = nodata_f64.map(|v| v as u8);
                let mut rgba = Vec::with_capacity(pixel_count * 4);
                for i in 0..pixel_count {
                    let base = i * 3;
                    if let (Some(&r), Some(&g), Some(&b)) =
                        (buf.get(base), buf.get(base + 1), buf.get(base + 2))
                    {
                        let alpha = if nodata_u8.is_some_and(|nd| r == nd && g == nd && b == nd) {
                            0
                        } else {
                            255
                        };
                        rgba.push(r);
                        rgba.push(g);
                        rgba.push(b);
                        rgba.push(alpha);
                    }
                }
                (rgba, RasterFormat::Rgba8)
            }
            (4, TypedArray::UInt8(_)) => {
                let mut buf = vec![0u8; (width * height) as usize * 4];
                stitch_tiles_u8(&decoded_tiles, &mut buf, width, height, tile_width, tile_height, 4);
                (buf, RasterFormat::Rgba8)
            }
            (3, TypedArray::UInt16(_)) => {
                let mut buf16 = vec![0u16; (width * height) as usize * 3];
                stitch_tiles_u16(
                    &decoded_tiles,
                    &mut buf16,
                    width,
                    height,
                    tile_width,
                    tile_height,
                    3,
                );
                let pixel_count = (width * height) as usize;
                let nodata_u16 = nodata_f64.map(|v| v as u16);
                let (min_val, max_val) = min_max_u16(&buf16, nodata_u16);
                let mut rgba = Vec::with_capacity(pixel_count * 4);
                for i in 0..pixel_count {
                    let base = i * 3;
                    let r = buf16[base];
                    let g = buf16[base + 1];
                    let b = buf16[base + 2];
                    let alpha = if nodata_u16.is_some_and(|nd| r == nd && g == nd && b == nd) {
                        0
                    } else {
                        255
                    };
                    rgba.push(normalize_u16(r, min_val, max_val));
                    rgba.push(normalize_u16(g, min_val, max_val));
                    rgba.push(normalize_u16(b, min_val, max_val));
                    rgba.push(alpha);
                }
                (rgba, RasterFormat::Rgba8)
            }
            (_, TypedArray::UInt8(_)) => {
                let spp = samples_per_pixel as usize;
                let mut buf = vec![0u8; (width * height) as usize * spp];
                stitch_tiles_u8(
                    &decoded_tiles,
                    &mut buf,
                    width,
                    height,
                    tile_width,
                    tile_height,
                    samples_per_pixel.into(),
                );
                let pixel_count = (width * height) as usize;
                let first_chan: Vec<u8> = (0..pixel_count).map(|i| buf[i * spp]).collect();
                let nodata_u8 = nodata_f64.map(|v| v as u8);
                let (min_val, max_val) = min_max_u8(&first_chan, nodata_u8);
                let data: Vec<u8> = first_chan.iter().map(|&v| normalize_u8(v, min_val, max_val)).collect();
                (data, RasterFormat::R8)
            }
            (_, TypedArray::UInt16(_)) => {
                let spp = samples_per_pixel as usize;
                let mut buf16 = vec![0u16; (width * height) as usize * spp];
                stitch_tiles_u16(
                    &decoded_tiles,
                    &mut buf16,
                    width,
                    height,
                    tile_width,
                    tile_height,
                    samples_per_pixel.into(),
                );
                let pixel_count = (width * height) as usize;
                let first_chan: Vec<u16> = (0..pixel_count).map(|i| buf16[i * spp]).collect();
                let nodata_u16 = nodata_f64.map(|v| v as u16);
                let (min_val, max_val) = min_max_u16(&first_chan, nodata_u16);
                let data: Vec<u8> = first_chan.iter().map(|&v| normalize_u16(v, min_val, max_val)).collect();
                (data, RasterFormat::R8)
            }
            _ => return Err(Error::UnsupportedColorType),
        };

        Ok(Raster {
            width,
            height,
            data,
            format,
            extent,
        })
    }
}

/// Convert tile data from planar layout [bands, height, width] to chunky layout [height, width, bands].
fn planar_to_chunky(data: TypedArray, bands: usize, height: usize, width: usize) -> TypedArray {
    match data {
        TypedArray::UInt8(planar) => {
            let mut chunky = vec![0u8; planar.len()];
            for b in 0..bands {
                for h in 0..height {
                    for w in 0..width {
                        chunky[(h * width + w) * bands + b] =
                            planar[b * height * width + h * width + w];
                    }
                }
            }
            TypedArray::UInt8(chunky)
        }
        TypedArray::UInt16(planar) => {
            let mut chunky = vec![0u16; planar.len()];
            for b in 0..bands {
                for h in 0..height {
                    for w in 0..width {
                        chunky[(h * width + w) * bands + b] =
                            planar[b * height * width + h * width + w];
                    }
                }
            }
            TypedArray::UInt16(chunky)
        }
        other => other,
    }
}

fn normalize_u8(val: u8, min: u8, max: u8) -> u8 {
    if min == 0 && max == 255 {
        return val;
    }
    if min >= max {
        return val;
    }
    ((val.saturating_sub(min) as u16 * 255) / (max - min) as u16) as u8
}

fn normalize_u16(val: u16, min: u16, max: u16) -> u8 {
    if min >= max {
        return (val >> 8) as u8;
    }
    ((val.saturating_sub(min) as u32 * 255) / (max - min) as u32) as u8
}

fn min_max_u8(data: &[u8], nodata: Option<u8>) -> (u8, u8) {
    let mut min = u8::MAX;
    let mut max = u8::MIN;
    for &v in data {
        if nodata.is_some_and(|nd| v == nd) {
            continue;
        }
        min = min.min(v);
        max = max.max(v);
    }
    if min > max {
        (0, 255)
    } else {
        (min, max)
    }
}

fn min_max_u16(data: &[u16], nodata: Option<u16>) -> (u16, u16) {
    let mut min = u16::MAX;
    let mut max = u16::MIN;
    for &v in data {
        if nodata.is_some_and(|nd| v == nd) {
            continue;
        }
        min = min.min(v);
        max = max.max(v);
    }
    if min > max {
        (0, 65535)
    } else {
        (min, max)
    }
}

fn stitch_tiles_u8(
    tiles: &[(usize, usize, TypedArray, [usize; 3])],
    buf: &mut [u8],
    image_width: u32,
    image_height: u32,
    tile_width: u32,
    tile_height: u32,
    channels: u32,
) {
    for (tx, ty, data, shape) in tiles {
        if let TypedArray::UInt8(tile_data) = data {
            let src_w = shape[1] as u32;

            let tile_x_offset = (*tx as u32) * tile_width;
            let tile_y_offset = (*ty as u32) * tile_height;
            let copy_w = src_w.min(image_width.saturating_sub(tile_x_offset));
            let copy_h = (shape[0] as u32).min(image_height.saturating_sub(tile_y_offset));

            for row in 0..copy_h {
                let src_start = (row * src_w * channels) as usize;
                let dst_start =
                    ((tile_y_offset + row) * image_width + tile_x_offset) as usize * channels as usize;
                let copy_len = (copy_w * channels) as usize;
                if let (Some(src), Some(dst)) = (
                    tile_data.get(src_start..src_start + copy_len),
                    buf.get_mut(dst_start..dst_start + copy_len),
                ) {
                    dst.copy_from_slice(src);
                }
            }
        }
    }
}

fn stitch_tiles_u16(
    tiles: &[(usize, usize, TypedArray, [usize; 3])],
    buf: &mut [u16],
    image_width: u32,
    image_height: u32,
    tile_width: u32,
    tile_height: u32,
    channels: u32,
) {
    for (tx, ty, data, shape) in tiles {
        if let TypedArray::UInt16(tile_data) = data {
            let src_w = shape[1] as u32;

            let tile_x_offset = (*tx as u32) * tile_width;
            let tile_y_offset = (*ty as u32) * tile_height;
            let copy_w = src_w.min(image_width.saturating_sub(tile_x_offset));
            let copy_h = (shape[0] as u32).min(image_height.saturating_sub(tile_y_offset));

            for row in 0..copy_h {
                let src_start = (row * src_w * channels) as usize;
                let dst_start =
                    ((tile_y_offset + row) * image_width + tile_x_offset) as usize * channels as usize;
                let copy_len = (copy_w * channels) as usize;
                if let (Some(src), Some(dst)) = (
                    tile_data.get(src_start..src_start + copy_len),
                    buf.get_mut(dst_start..dst_start + copy_len),
                ) {
                    dst.copy_from_slice(src);
                }
            }
        }
    }
}
