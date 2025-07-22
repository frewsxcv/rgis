use crate::{Error, Raster};
use geo_raster::RasterFormat;
use std::io::Cursor;
use tiff::{
    decoder::{Decoder, DecodingResult},
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
        })
    }
}
