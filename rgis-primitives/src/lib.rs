use std::num;

#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, bevy::ecs::component::Component,
)]
pub struct LayerId(num::NonZeroU16);

impl LayerId {
    /// Creates a `LayerId` from a `u16` value.
    ///
    /// # Panics
    ///
    /// Panics if `value` is 0.
    pub fn from_u16(value: u16) -> Self {
        LayerId(
            num::NonZeroU16::new(value).expect("LayerId value must be non-zero"),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    GeoJson,
    Wkt,
}

impl ExportFormat {
    pub fn extension(self) -> &'static str {
        match self {
            ExportFormat::GeoJson => "geojson",
            ExportFormat::Wkt => "wkt",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ExportFormat::GeoJson => "GeoJSON",
            ExportFormat::Wkt => "WKT",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crs {
    pub epsg_code: Option<u16>,
    pub proj_string: Option<String>,
    pub op_handle: geodesy::ctx::OpHandle,
}
