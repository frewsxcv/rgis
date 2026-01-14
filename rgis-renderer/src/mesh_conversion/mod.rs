use bevy::prelude::{info_span, Mesh};
use build_mesh::BuildMesh;
use geo_traits::*;
use line_string::LineStringMeshBuilder;
use polygon::PolygonMeshBuilder;
use std::iter;

pub use point::SpritePosition;
pub use polygon::PolygonMesh;

mod build_mesh;
mod line_string;
mod point;
mod polygon;

pub fn geometry_to_mesh<Scalar: geo_types::CoordFloat>(
    geometry: impl GeometryTrait<T = Scalar>,
) -> Result<GeometryMesh, Error> {
    let mut ctx = build_mesh::BuildBevyMeshesContext::default();

    info_span!("Populating Bevy mesh builder")
        .in_scope(|| build_mesh::populate_geometry_mesh_builders(&geometry, &mut ctx))?;

    info_span!("Building Bevy meshes").in_scope(|| {
        [
            ctx.point_mesh_builder.build(),
            ctx.line_string_mesh_builder.build(),
            ctx.polygon_mesh_builder.build(),
        ]
        .into_iter()
        .find(|prepared_mesh| prepared_mesh.is_ok())
        .unwrap_or(Err(Error::CouldNotBuildMesh))
    })
}

pub enum GeometryMesh {
    Point(Vec<SpritePosition>),
    LineString(Mesh),
    Polygon(polygon::PolygonMesh),
}

#[derive(Debug)]
pub enum Error {
    CouldNotBuildMesh,
    CouldNotConvertToF32,
    EmptyGeometry,
    BevyEarcutr(bevy_earcutr::Error),
}
