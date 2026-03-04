use std::ops::Range;

use async_tiff::decoder::DecoderRegistry;
use async_tiff::error::AsyncTiffResult;
use async_tiff::metadata::TiffMetadataReader;
use async_tiff::reader::AsyncFileReader;
use async_tiff::{Array, TypedArray};

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

        // Read geo-referencing tags
        let tiepoint = ifd.model_tiepoint().ok_or(Error::MissingGeoInfo)?;
        let scale = ifd.model_pixel_scale().ok_or(Error::MissingGeoInfo)?;

        let origin_x = *tiepoint.get(3).ok_or(Error::MissingGeoInfo)?;
        let origin_y = *tiepoint.get(4).ok_or(Error::MissingGeoInfo)?;
        let scale_x = *scale.first().ok_or(Error::MissingGeoInfo)?;
        let scale_y = *scale.get(1).ok_or(Error::MissingGeoInfo)?;

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

        // Fetch and decode all tiles
        let mut decoded_tiles = Vec::with_capacity(tiles_x * tiles_y);
        for ty in 0..tiles_y {
            for tx in 0..tiles_x {
                let tile = ifd.fetch_tile(tx, ty, &reader).await?;
                let array = tile.decode(&decoder_registry)?;
                decoded_tiles.push((tx, ty, array));
            }
        }

        // Determine pixel format from first tile and stitch
        let first_data = decoded_tiles
            .first()
            .ok_or(Error::UnsupportedColorType)?
            .2
            .data();

        let (data, format) = match (samples_per_pixel, first_data) {
            (1, TypedArray::UInt8(_)) => {
                let mut buf = vec![0u8; (width * height) as usize];
                stitch_tiles_u8(&decoded_tiles, &mut buf, width, height, tile_width, tile_height, 1);
                (buf, RasterFormat::R8)
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
                let data: Vec<u8> = buf16.iter().map(|v| (v >> 8) as u8).collect();
                (data, RasterFormat::R8)
            }
            (3, TypedArray::UInt8(_)) => {
                let mut buf = vec![0u8; (width * height) as usize * 3];
                stitch_tiles_u8(&decoded_tiles, &mut buf, width, height, tile_width, tile_height, 3);
                let pixel_count = (width * height) as usize;
                let mut rgba = Vec::with_capacity(pixel_count * 4);
                for i in 0..pixel_count {
                    let base = i * 3;
                    if let (Some(&r), Some(&g), Some(&b)) =
                        (buf.get(base), buf.get(base + 1), buf.get(base + 2))
                    {
                        rgba.push(r);
                        rgba.push(g);
                        rgba.push(b);
                        rgba.push(255);
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
                let mut rgba = Vec::with_capacity(pixel_count * 4);
                for i in 0..pixel_count {
                    let base = i * 3;
                    rgba.push((buf16[base] >> 8) as u8);
                    rgba.push((buf16[base + 1] >> 8) as u8);
                    rgba.push((buf16[base + 2] >> 8) as u8);
                    rgba.push(255);
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
                let data: Vec<u8> = (0..pixel_count).map(|i| buf[i * spp]).collect();
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
                let data: Vec<u8> =
                    (0..pixel_count).map(|i| (buf16[i * spp] >> 8) as u8).collect();
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

fn stitch_tiles_u8(
    tiles: &[(usize, usize, Array)],
    buf: &mut [u8],
    image_width: u32,
    image_height: u32,
    tile_width: u32,
    tile_height: u32,
    channels: u32,
) {
    for (tx, ty, array) in tiles {
        if let TypedArray::UInt8(tile_data) = array.data() {
            let shape = array.shape();
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
    tiles: &[(usize, usize, Array)],
    buf: &mut [u16],
    image_width: u32,
    image_height: u32,
    tile_width: u32,
    tile_height: u32,
    channels: u32,
) {
    for (tx, ty, array) in tiles {
        if let TypedArray::UInt16(tile_data) = array.data() {
            let shape = array.shape();
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
