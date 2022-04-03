#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy_render::prelude::*;
use geo::algorithm::coords_iter::CoordsIter;
use std::error;

mod line_string;
mod point;

type Vertex = [f32; 3]; // [x, y, z]

fn build_mesh_from_vertices(
    primitive_topology: bevy_render::render_resource::PrimitiveTopology,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
) -> Mesh {
    let num_vertices = vertices.len();
    let mut mesh = Mesh::new(primitive_topology);
    mesh.set_indices(Some(bevy_render::mesh::Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let normals = vec![[0.0, 0.0, 0.0]; num_vertices];
    let uvs = vec![[0.0, 0.0]; num_vertices];

    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

pub struct BuildBevyMeshesContext {
    point_mesh_builder: point::PointMeshBuilder,
    line_string_mesh_builder: line_string::LineStringMeshBuilder,
    polygon_mesh_builder: bevy_earcutr::PolygonMeshBuilder,
}

impl BuildBevyMeshesContext {
    pub fn new() -> Self {
        BuildBevyMeshesContext {
            point_mesh_builder: point::PointMeshBuilder::new(),
            line_string_mesh_builder: line_string::LineStringMeshBuilder::new(),
            polygon_mesh_builder: bevy_earcutr::PolygonMeshBuilder::new(),
        }
    }
}

pub fn build_bevy_meshes<G: BuildBevyMeshes>(
    geo: &G,
    mut ctx: BuildBevyMeshesContext,
) -> Result<impl Iterator<Item = Mesh>, Box<dyn error::Error>> {
    geo.populate_mesh_builders(&mut ctx)?;

    Ok([
        ctx.point_mesh_builder.build(),
        ctx.line_string_mesh_builder.build(),
        ctx.polygon_mesh_builder.build(),
    ]
    .into_iter()
    .flatten())
}

pub trait BuildBevyMeshes {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>>;
}

impl BuildBevyMeshes for geo::Point<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        ctx.point_mesh_builder.add_point(self)
    }
}

impl BuildBevyMeshes for geo::LineString<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        ctx.line_string_mesh_builder.add_line_string(self)
    }
}

impl BuildBevyMeshes for geo::Polygon<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        ctx.polygon_mesh_builder
            .add_earcutr_input(polygon_to_earcutr_input(self));
        Ok(())
    }
}

impl BuildBevyMeshes for geo::MultiPoint<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        for point in &self.0 {
            point.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl BuildBevyMeshes for geo::MultiLineString<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        for line_string in &self.0 {
            line_string.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl BuildBevyMeshes for geo::MultiPolygon<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        for polygon in &self.0 {
            polygon.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl BuildBevyMeshes for geo::Line<f64> {
    fn populate_mesh_builders(
        &self,
        _ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }
}

impl BuildBevyMeshes for geo::Triangle<f64> {
    fn populate_mesh_builders(
        &self,
        _ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }
}

impl BuildBevyMeshes for geo::Rect<f64> {
    fn populate_mesh_builders(
        &self,
        _ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }
}

impl BuildBevyMeshes for geo::Geometry<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        match self {
            geo::Geometry::Point(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::Line(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::LineString(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::Polygon(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::MultiPoint(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::MultiLineString(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::MultiPolygon(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::GeometryCollection(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::Triangle(g) => g.populate_mesh_builders(ctx),
            geo::Geometry::Rect(g) => g.populate_mesh_builders(ctx),
        }
    }
}

impl BuildBevyMeshes for geo::GeometryCollection<f64> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), Box<dyn error::Error>> {
        for g in self {
            g.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

fn polygon_to_earcutr_input(polygon: &geo::Polygon<f64>) -> bevy_earcutr::EarcutrInput {
    let mut vertices = Vec::with_capacity(polygon.coords_count() * 2);
    let mut interior_indices = Vec::with_capacity(polygon.interiors().len());

    flat_line_string_coords_2(polygon.exterior(), &mut vertices);

    for interior in polygon.interiors() {
        interior_indices.push(vertices.len() / 2);
        flat_line_string_coords_2(interior, &mut vertices);
    }

    bevy_earcutr::EarcutrInput {
        vertices,
        interior_indices,
    }
}

fn flat_line_string_coords_2(line_string: &geo::LineString<f64>, vertices: &mut Vec<f64>) {
    for coord in &line_string.0 {
        vertices.push(coord.x);
        vertices.push(coord.y);
    }
}
