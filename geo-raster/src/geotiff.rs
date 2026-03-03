use std::io::Cursor;
use tiff::decoder::{Decoder, DecodingResult};
use tiff::tags::Tag;
use tiff::ColorType;

use crate::{Error, Raster, RasterFormat};

pub struct GeoTiffSource {
    bytes: bytes::Bytes,
}

impl GeoTiffSource {
    pub fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoTiffSource { bytes }
    }

    pub fn load(self) -> Result<Raster, Error> {
        let cursor = Cursor::new(self.bytes.as_ref());
        let mut decoder = Decoder::new(cursor)?;

        let (width, height) = decoder.dimensions()?;

        // Read geo-referencing tags
        // ModelTiepointTag (33922) and ModelPixelScaleTag (33550)
        let (origin_x, origin_y) = read_tiepoint(&mut decoder)?;
        let (scale_x, scale_y) = read_pixel_scale(&mut decoder)?;

        let min_x = origin_x;
        let max_y = origin_y;
        let max_x = origin_x + scale_x * f64::from(width);
        let min_y = origin_y - scale_y * f64::from(height);

        let extent = geo_types::Rect::new(
            geo_types::coord! { x: min_x, y: min_y },
            geo_types::coord! { x: max_x, y: max_y },
        );

        let color_type = decoder.colortype()?;
        let image = decoder.read_image()?;

        let (data, format) = match (color_type, image) {
            (ColorType::Gray(8), DecodingResult::U8(buf)) => {
                (buf, RasterFormat::R8)
            }
            (ColorType::Gray(16), DecodingResult::U16(buf)) => {
                // Downscale 16-bit to 8-bit
                let data: Vec<u8> = buf.iter().map(|v| (v >> 8) as u8).collect();
                (data, RasterFormat::R8)
            }
            (ColorType::RGB(8), DecodingResult::U8(buf)) => {
                // Convert RGB to RGBA
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
            (ColorType::RGBA(8), DecodingResult::U8(buf)) => {
                (buf, RasterFormat::Rgba8)
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

fn read_tiepoint<R: std::io::Read + std::io::Seek>(
    decoder: &mut Decoder<R>,
) -> Result<(f64, f64), Error> {
    // ModelTiepointTag: 6 doubles (I, J, K, X, Y, Z)
    // We want X (index 3) and Y (index 4)
    match decoder.get_tag_f64_vec(Tag::ModelTiepointTag) {
        Ok(values) => {
            let x = values.get(3).copied().ok_or(Error::MissingGeoInfo)?;
            let y = values.get(4).copied().ok_or(Error::MissingGeoInfo)?;
            Ok((x, y))
        }
        _ => Err(Error::MissingGeoInfo),
    }
}

fn read_pixel_scale<R: std::io::Read + std::io::Seek>(
    decoder: &mut Decoder<R>,
) -> Result<(f64, f64), Error> {
    // ModelPixelScaleTag: 3 doubles (ScaleX, ScaleY, ScaleZ)
    match decoder.get_tag_f64_vec(Tag::ModelPixelScaleTag) {
        Ok(values) => {
            let sx = values.first().copied().ok_or(Error::MissingGeoInfo)?;
            let sy = values.get(1).copied().ok_or(Error::MissingGeoInfo)?;
            Ok((sx, sy))
        }
        _ => Err(Error::MissingGeoInfo),
    }
}
