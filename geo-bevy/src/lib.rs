use bevy_render::prelude::*;
use geo::algorithm::coords_iter::CoordsIter;
use std::convert::TryFrom;

struct LineStringMeshBuilder {
    vertices: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl LineStringMeshBuilder {
    fn new() -> Self {
        // TODO: capacity?
        LineStringMeshBuilder {
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    fn add_line_string(&mut self, line_string: &geo::LineString<f64>) {
        let index_base = self.vertices.len();
        for (i, coord) in line_string.0.iter().enumerate() {
            self.vertices.push([coord.x as f32, coord.y as f32]);
            if i != line_string.0.len() - 1 {
                self.indices.push(u32::try_from(index_base + i).unwrap());
                self.indices
                    .push(u32::try_from(index_base + i + 1).unwrap());
            }
        }
    }

    pub fn build(self) -> Mesh {
        build_mesh_from_vertices(
            bevy_render::pipeline::PrimitiveTopology::LineList,
            self.vertices,
            self.indices,
        )
    }
}

struct PointMeshBuilder {
    vertices: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl PointMeshBuilder {
    fn new() -> Self {
        // TODO: capacity?
        PointMeshBuilder {
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    fn add_point(&mut self, point: &geo::Point<f64>) {
        let index_base = self.vertices.len();
        self.vertices.push([point.x() as f32, point.y() as f32]);
        self.indices.push(u32::try_from(index_base).unwrap());
    }

    pub fn build(self) -> Mesh {
        build_mesh_from_vertices(
            bevy_render::pipeline::PrimitiveTopology::PointList,
            self.vertices,
            self.indices,
        )
    }
}

fn build_mesh_from_vertices(
    primitive_topology: bevy_render::pipeline::PrimitiveTopology,
    vertices: Vec<[f32; 2]>,
    indices: Vec<u32>,
) -> Mesh {
    let num_vertices = vertices.len();
    let mut mesh = Mesh::new(primitive_topology);
    mesh.set_indices(Some(bevy_render::mesh::Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let mut normals = Vec::new();
    normals.resize(num_vertices, [0.0, 0.0, 0.0]);
    let mut uvs = Vec::new();
    uvs.resize(num_vertices, [0.0, 0.0]);

    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

pub struct BuildBevyMeshesContext {
    point_mesh_builder: PointMeshBuilder,
    line_string_mesh_builder: LineStringMeshBuilder,
    polygon_mesh_builder: bevy_earcutr::PolygonMeshBuilder,
}

impl BuildBevyMeshesContext {
    pub fn new() -> Self {
        BuildBevyMeshesContext {
            point_mesh_builder: PointMeshBuilder::new(),
            line_string_mesh_builder: LineStringMeshBuilder::new(),
            polygon_mesh_builder: bevy_earcutr::PolygonMeshBuilder::new(),
        }
    }
}

pub trait BuildBevyMeshes {
    fn build_bevy_meshes(&self, mut ctx: BuildBevyMeshesContext) -> Vec<Mesh> {
        let mut meshes = vec![];

        self.populate_mesh_builders(&mut ctx);

        // TODO: do the builders handle the empty case?

        meshes.push(ctx.point_mesh_builder.build());
        meshes.push(ctx.line_string_mesh_builder.build());
        meshes.push(ctx.polygon_mesh_builder.build());

        meshes
    }

    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext);
}

impl BuildBevyMeshes for geo::Point<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        ctx.point_mesh_builder.add_point(self);
    }
}

impl BuildBevyMeshes for geo::LineString<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        ctx.line_string_mesh_builder.add_line_string(self);
    }
}

impl BuildBevyMeshes for geo::Polygon<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        ctx.polygon_mesh_builder
            .add_earcutr_input(polygon_to_earcutr_input(self));
    }
}

impl BuildBevyMeshes for geo::MultiPoint<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for point in &self.0 {
            point.populate_mesh_builders(ctx);
        }
    }
}

impl BuildBevyMeshes for geo::MultiLineString<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for line_string in &self.0 {
            line_string.populate_mesh_builders(ctx);
        }
    }
}

impl BuildBevyMeshes for geo::MultiPolygon<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for polygon in &self.0 {
            polygon.populate_mesh_builders(ctx);
        }
    }
}

impl BuildBevyMeshes for geo::Line<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        unimplemented!()
    }
}

impl BuildBevyMeshes for geo::Triangle<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        unimplemented!()
    }
}

impl BuildBevyMeshes for geo::Rect<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        unimplemented!()
    }
}

impl BuildBevyMeshes for geo::Geometry<f64> {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
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
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for g in self {
            g.populate_mesh_builders(ctx);
        }
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
