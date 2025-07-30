use std::sync;

pub struct Entry {
    pub name: &'static str,
    pub url: &'static str,
    pub crs: u16,
}

pub struct Folder {
    pub name: &'static str,
    pub entries: Vec<Entry>,
}

fn build() -> Vec<Folder> {
    vec![
        Folder {
            name: "Russia",
            entries: vec![
                Entry {
                    name: "Country",
                    url: "https://raw.githubusercontent.com/hugoledoux/BIGpolygons/master/russia.geojson",
                    crs: 4326,
                },
            ],
        },
        Folder {
            name: "USA",
            entries: vec![
                Entry {
                    name: "States",
                    url: "https://raw.githubusercontent.com/PublicaMundi/MappingAPI/master/data/geojson/us-states.json",
                    crs: 4326,
                },
            ],
        },
        Folder {
            name: "USA: Amtrak",
            entries: vec![
                Entry {
                    name: "Stations",
                    url: "https://raw.githubusercontent.com/datanews/amtrak-geojson/master/amtrak-stations.geojson",
                    crs: 4326,
                },
                Entry {
                    name: "Tracks",
                    url: "https://raw.githubusercontent.com/datanews/amtrak-geojson/master/amtrak-track.geojson",
                    crs: 4326,
                },
            ],
        },
        Folder {
            name: "USA: New York City",
            entries: vec![
                Entry {
                    name: "Parks",
                    url: "https://storage.googleapis.com/rgis-library/NYC/Parks.geojson",
                    crs: 4326,
                },
                Entry {
                    name: "Shoreline",
                    url: "https://storage.googleapis.com/rgis-library/NYC/Shoreline.geojson",
                    crs: 4326,
                },
                Entry {
                    name: "Sidewalk",
                    url: "https://storage.googleapis.com/rgis-library/NYC/Sidewalk.geojson",
                    crs: 4326,
                },
                Entry {
                    name: "Street Centerline",
                    url: "https://storage.googleapis.com/rgis-library/NYC/Street-Centerline.geojson",
                    crs: 4326,
                },
                Entry {
                    name: "Subway Lines",
                    url: "https://storage.googleapis.com/rgis-library/NYC/Subway-Lines.geojson",
                    crs: 4326,
                },
            ],
        },
        Folder {
            name: "USA: Washington D.C.",
            entries: vec![
                Entry {
                    name: "Street Centerline",
                    url: "https://raw.githubusercontent.com/benbalter/dc-maps/master/maps/street-centerlines.geojson",
                    crs: 4326,
                }
            ],
        },
        Folder {
            name: "World",
            entries: vec![
                Entry {
                    name: "Countries",
                    url: "https://raw.githubusercontent.com/andrea-ballatore/open-geo-data-education/main/datasets/world_country_boundaries_2018/natural_earth_world_boundaries_50m_2018.geojson",
                    crs: 4326,
                }, Entry {
                    name: "Lakes",
                    url: "https://raw.githubusercontent.com/simonepri/geo-maps/master/previews/earth-lakes.geo.json",
                    crs: 4326,
                },
                Entry {
                    name: "Tectonic Plate Boundaries",
                    url: "https://raw.githubusercontent.com/andrea-ballatore/open-geo-data-education/main/datasets/tectonic_plates_2002/PB2002_boundaries.geojson",
                    crs: 4326,
                }
            ],
        },
    ]
}

pub fn get() -> &'static Vec<Folder> {
    static LIBRARY: sync::OnceLock<Vec<Folder>> = sync::OnceLock::new();
    LIBRARY.get_or_init(build)
}
