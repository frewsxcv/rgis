use bevy::{
    prelude::Mesh,
    render::mesh::{MeshVertexAttribute, VertexFormat},
};
use geo_traits::CoordTrait;
use num_traits::cast::ToPrimitive;

type Point = [f32; 3]; // [x, y, z]

pub const ATTRIBUTE_POINT_A: MeshVertexAttribute =
    MeshVertexAttribute::new("PointA", 1, VertexFormat::Float32x3);
pub const ATTRIBUTE_POINT_B: MeshVertexAttribute =
    MeshVertexAttribute::new("PointB", 2, VertexFormat::Float32x3);

#[derive(Default)]
pub struct LineStringMeshBuilder {
    positions: Vec<Point>,
    point_as: Vec<Point>,
    point_bs: Vec<Point>,
    indices: Vec<u32>,
}

impl LineStringMeshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_coords<I, C>(&mut self, coords: I) -> Result<(), crate::Error>
    where
        I: Iterator<Item = C>,
        C: CoordTrait,
    {
        let mut index_base = self.positions.len() as u32;

        let mut last_coord = None;
        for coord in coords {
            if let Some(last_coord) = last_coord {
                let a = [
                    last_coord
                        .x()
                        .to_f32()
                        .ok_or(crate::Error::CouldNotConvertToF32)?,
                    last_coord
                        .y()
                        .to_f32()
                        .ok_or(crate::Error::CouldNotConvertToF32)?,
                    0.0,
                ];
                let b = [
                    coord
                        .x()
                        .to_f32()
                        .ok_or(crate::Error::CouldNotConvertToF32)?,
                    coord
                        .y()
                        .to_f32()
                        .ok_or(crate::Error::CouldNotConvertToF32)?,
                    0.0,
                ];

                self.positions.push([0.0, -0.5, 0.0]);
                self.positions.push([1.0, -0.5, 0.0]);
                self.positions.push([1.0, 0.5, 0.0]);
                self.positions.push([0.0, 0.5, 0.0]);

                for _ in 0..4 {
                    self.point_as.push(a);
                    self.point_bs.push(b);
                }

                self.indices.push(index_base);
                self.indices.push(index_base + 1);
                self.indices.push(index_base + 2);
                self.indices.push(index_base);
                self.indices.push(index_base + 2);
                self.indices.push(index_base + 3);

                index_base += 4;
            }
            last_coord = Some(coord);
        }

        Ok(())
    }
}

impl TryFrom<LineStringMeshBuilder> for Mesh {
    type Error = crate::Error;

    fn try_from(
        line_string_mesh_builder: LineStringMeshBuilder,
    ) -> Result<Self, Self::Error> {
        if line_string_mesh_builder.positions.is_empty() {
            Err(crate::Error::EmptyGeometry)
        } else {
            let mut mesh = Mesh::new(
                bevy::render::render_resource::PrimitiveTopology::TriangleList,
                Default::default(),
            );
            mesh.insert_indices(bevy::render::mesh::Indices::U32(
                line_string_mesh_builder.indices,
            ));
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                line_string_mesh_builder.positions,
            );
            mesh.insert_attribute(ATTRIBUTE_POINT_A, line_string_mesh_builder.point_as);
            mesh.insert_attribute(ATTRIBUTE_POINT_B, line_string_mesh_builder.point_bs);
            Ok(mesh)
        }
    }
}

impl crate::build_mesh::BuildMesh for LineStringMeshBuilder {
    fn build(self) -> Result<crate::GeometryMesh, crate::Error> {
        Ok(crate::GeometryMesh::LineString(self.try_into()?))
    }
}
