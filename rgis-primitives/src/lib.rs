use std::num;
use std::sync::atomic::{AtomicU16, Ordering};

/// Named system sets for cross-crate ordering of Bevy systems.
///
/// The intended ordering is:
///   `FileLoading` → `LayerProcessing` → `Transform` → `Rendering` → `Camera`
///
/// This prevents race conditions such as newly-projected geometry meshes being
/// deleted by a stale despawn that runs out of order.
#[derive(bevy::ecs::schedule::SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RgisSet {
    /// Loading files and creating layer entities.
    FileLoading,
    /// Processing layer data (creation, deletion, visibility, CRS updates).
    LayerProcessing,
    /// Reprojecting geometry / raster data into the target CRS.
    Transform,
    /// Building and spawning render meshes / sprites.
    Rendering,
    /// Updating the camera (centering, panning, zooming).
    Camera,
}

#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, bevy::ecs::component::Component,
)]
pub struct LayerId(num::NonZeroU16);

static NEXT_LAYER_ID: AtomicU16 = AtomicU16::new(1);

impl LayerId {
    /// Creates a new unique `LayerId`.
    pub fn new() -> Self {
        let value = NEXT_LAYER_ID.fetch_add(1, Ordering::Relaxed);
        LayerId(
            num::NonZeroU16::new(value).expect("LayerId overflow"),
        )
    }

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

/// Geographic bounding box defining the valid area of use for a CRS.
///
/// Coordinates are in degrees (WGS 84).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AreaOfUse {
    pub lat_south: f64,
    pub lon_west: f64,
    pub lat_north: f64,
    pub lon_east: f64,
}

/// Parse `BBOX[lat_south,lon_west,lat_north,lon_east]` from a WKT string.
fn parse_bbox_from_wkt(wkt: &str) -> Option<AreaOfUse> {
    let start = wkt.find("BBOX[")?;
    let after = &wkt[start + 5..];
    let end = after.find(']')?;
    let inner = &after[..end];
    let mut parts = inner.split(',');
    let lat_south = parts.next()?.parse().ok()?;
    let lon_west = parts.next()?.parse().ok()?;
    let lat_north = parts.next()?.parse().ok()?;
    let lon_east = parts.next()?.parse().ok()?;
    Some(AreaOfUse { lat_south, lon_west, lat_north, lon_east })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crs {
    pub epsg_code: Option<u16>,
    pub proj_string: Option<String>,
    pub op_handle: geodesy::ctx::OpHandle,
}

impl Crs {
    pub fn is_geographic(&self) -> bool {
        if let Some(code) = self.epsg_code {
            crs_definitions::from_code(code)
                .map(|def| def.proj4.contains("+proj=longlat"))
                .unwrap_or(true)
        } else if let Some(ref proj) = self.proj_string {
            proj.contains("+proj=longlat")
        } else {
            true
        }
    }

    /// Returns the geographic area of use for this CRS, if known.
    ///
    /// The bounds are parsed from the `BBOX` element in the WKT definition
    /// provided by the EPSG registry via the `crs-definitions` crate.
    pub fn area_of_use(&self) -> Option<AreaOfUse> {
        let code = self.epsg_code?;
        let def = crs_definitions::from_code(code)?;
        parse_bbox_from_wkt(def.wkt)
    }
}
