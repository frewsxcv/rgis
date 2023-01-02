#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub struct LibraryEntry {
    pub name: &'static str,
    pub url: &'static str,
    pub crs: &'static str,
}

pub const ENTRIES: [LibraryEntry; 8] = [LibraryEntry {
    name: "USA States",
    url: "https://raw.githubusercontent.com/PublicaMundi/MappingAPI/master/data/geojson/us-states.json",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "Tectonic Plate Boundaries",
    url: "https://raw.githubusercontent.com/andrea-ballatore/open-geo-data-education/main/datasets/tectonic_plates_2002/PB2002_boundaries.geojson",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "Countries",
    url: "https://raw.githubusercontent.com/andrea-ballatore/open-geo-data-education/main/datasets/world_country_boundaries_2018/natural_earth_world_boundaries_50m_2018.geojson",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "Amtrak Stations",
    url: "https://raw.githubusercontent.com/datanews/amtrak-geojson/master/amtrak-stations.geojson",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "Amtrak Tracks",
    url: "https://raw.githubusercontent.com/datanews/amtrak-geojson/master/amtrak-track.geojson",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "World lakes",
    url: "https://raw.githubusercontent.com/simonepri/geo-maps/master/previews/earth-lakes.geo.json",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "Russia",
    url: "https://raw.githubusercontent.com/hugoledoux/BIGpolygons/master/russia.geojson",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "Washington DC street centerlines",
    url: "https://raw.githubusercontent.com/benbalter/dc-maps/master/maps/street-centerlines.geojson",
    crs: "EPSG:4326",
}];
