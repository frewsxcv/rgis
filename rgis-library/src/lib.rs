pub struct LibraryEntry {
    pub name: &'static str,
    pub url: &'static str,
    pub crs: &'static str,
}

pub const ENTRIES: [LibraryEntry; 2] = [LibraryEntry {
    name: "USA States",
    url: "https://raw.githubusercontent.com/PublicaMundi/MappingAPI/master/data/geojson/us-states.json",
    crs: "EPSG:4326",
}, LibraryEntry {
    name: "Tectonic Plate Boundaries",
    url: "https://raw.githubusercontent.com/andrea-ballatore/open-geo-data-education/main/datasets/tectonic_plates_2002/PB2002_boundaries.geojson",
    crs: "EPSG:4326",
}];
