// WGS84 ellipsoid constants:
const WGS84_ELLIPSOID_A: f32 = 6378137.;
const WGS84_ELLIPSOID_E: f32 = 8.1819190842622e-2;

pub struct EcefCoord {
    x: f32,
    y: f32,
    z: f32,
}

impl EcefCoord {
    /// Will assume altitude is 0
    pub fn from_lng_lat(lng: f32, lat: f32) -> EcefCoord {
        EcefCoord::from_lng_lat_alt(lng, lat, 0.)
    }

    /// Section 4-14 of http://earth-info.nga.mil/GandG/publications/tr8350.2/wgs84fin.pdf
    pub fn from_lng_lat_alt(lng: f32, lat: f32, alt: f32) -> EcefCoord {
        // intermediate calculation
        // (prime vertical radius of curvature)
        let n = WGS84_ELLIPSOID_A / (1. - WGS84_ELLIPSOID_E.powi(2) * f32::sin(lat).powi(2)).sqrt();

        EcefCoord {
            x: (n + alt) * f32::cos(lat) * f32::cos(lng),
            y: (n + alt) * f32::cos(lat) * f32::sin(lng),
            z: ((1. - WGS84_ELLIPSOID_E.powi(2)) * n + alt) * f32::sin(lat),
        }
    }
}
