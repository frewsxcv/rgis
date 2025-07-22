use crate::{Error, Raster};
use geo_raster::RasterFormat;
use geo_types::{Coord, Rect};
use std::io::Cursor;
use tiff::{
    decoder::{Decoder, DecodingResult},
    tags::Tag,
    ColorType,
};

pub struct GeoTiffSource {
    bytes: bytes::Bytes,
}

impl crate::FileLoader for GeoTiffSource {
    type Output = Raster;

    fn from_bytes(bytes: bytes::Bytes) -> Self {
        Self { bytes }
    }

    fn load(self) -> Result<Self::Output, Error> {
        let mut cursor = Cursor::new(self.bytes);
        let mut decoder = Decoder::new(&mut cursor)?;
        let (width, height) = decoder.dimensions()?;
        let color_type = decoder.colortype()?;
        let model_tiepoint = decoder
            .get_tag_f64_vec(Tag::Unknown(33922))
            .ok()
            .and_then(|v| v.get(3..5).map(|s| (s[0], s[1])));

        let model_pixel_scale = decoder
            .get_tag_f64_vec(Tag::Unknown(33550))
            .ok()
            .and_then(|v| v.get(0..2).map(|s| (s[0], s[1])));

        let extent = if let (Some((tie_x, tie_y)), Some((scale_x, scale_y))) =
            (model_tiepoint, model_pixel_scale)
        {
            let min_x = tie_x;
            let max_y = tie_y;
            let max_x = min_x + (width as f64 * scale_x);
            let min_y = max_y - (height as f64 * scale_y);
            Rect::new(Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y })
        } else {
            // Default to a dummy extent if tags are not found
            Rect::new(
                Coord { x: 0.0, y: 0.0 },
                Coord {
                    x: width as f64,
                    y: height as f64,
                },
            )
        };
        let image_data = match decoder.read_image()? {
            DecodingResult::U8(data) => data,
            // For now, just handle U8 and convert others, losing precision.
            // This can be improved later to handle different data types properly.
            DecodingResult::U16(data) => data.into_iter().map(|p| (p / 256) as u8).collect(),
            DecodingResult::U32(data) => data.into_iter().map(|p| (p / 16777216) as u8).collect(),
            DecodingResult::U64(data) => data
                .into_iter()
                .map(|p| (p / 72057594037927936) as u8)
                .collect(),
            DecodingResult::F32(data) => data.into_iter().map(|p| (p * 255.0) as u8).collect(),
            DecodingResult::F64(data) => data.into_iter().map(|p| (p * 255.0) as u8).collect(),
            DecodingResult::I8(data) => data.into_iter().map(|p| p as u8).collect(),
            DecodingResult::I16(data) => data.into_iter().map(|p| (p / 256) as u8).collect(),
            DecodingResult::I32(data) => data.into_iter().map(|p| (p / 16777216) as u8).collect(),
            DecodingResult::I64(data) => data
                .into_iter()
                .map(|p| (p / 72057594037927936) as u8)
                .collect(),
        };

        let (format, data) = match color_type {
            ColorType::Gray(_) => (RasterFormat::R8, image_data),
            ColorType::RGB(_) => {
                let mut rgba_data = Vec::with_capacity(image_data.len() / 3 * 4);
                for chunk in image_data.chunks_exact(3) {
                    rgba_data.extend_from_slice(chunk);
                    rgba_data.push(255); // Alpha channel
                }
                (RasterFormat::Rgba8, rgba_data)
            }
            ColorType::RGBA(_) => (RasterFormat::Rgba8, image_data),
            _ => panic!("Unsupported color type"), // TODO: handle this more gracefully
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
