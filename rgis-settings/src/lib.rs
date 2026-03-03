use bevy::prelude::*;

mod systems;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tool {
    Pan,
    Query,
    Measure,
}

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub enum DistanceMethod {
    #[default]
    Haversine,
    Geodesic,
    Rhumb,
}

impl DistanceMethod {
    pub fn label(&self) -> &'static str {
        match self {
            DistanceMethod::Haversine => "Haversine",
            DistanceMethod::Geodesic => "Geodesic",
            DistanceMethod::Rhumb => "Rhumb",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DistanceMethod::Haversine => "Great-circle distance using the Haversine formula",
            DistanceMethod::Geodesic => "Geodesic distance on the WGS84 ellipsoid (most accurate)",
            DistanceMethod::Rhumb => "Distance along a rhumb line (constant bearing)",
        }
    }
}

#[derive(Resource)]
pub struct RgisSettings {
    pub current_tool: Tool,
    pub show_scale: bool,
    pub distance_method: DistanceMethod,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RgisSettings {
            current_tool: Tool::Pan,
            show_scale: true,
            distance_method: DistanceMethod::default(),
        });
    }
}
