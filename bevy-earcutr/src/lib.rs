use bevy::prelude::*;
use std::convert::TryFrom;

type EarcutrIndices = Vec<usize>;
type EarcutrVertices = Vec<f64>;
type BevyIndices = Vec<u32>;
type BevyVertices = Vec<[f32; 2]>;

#[derive(Debug)]
pub struct EarcutrInput {
    pub vertices: EarcutrVertices,
    pub interior_indices: EarcutrIndices,
}

#[derive(Debug)]
pub struct EarcutrResult {
    pub vertices: EarcutrVertices,
    pub triangle_indices: EarcutrIndices,
}

impl EarcutrResult {
    fn merge(&mut self, mut other: EarcutrResult) {
        let base_triangle_index = self.vertices.len() / 2;
        for other_triangle_index in other.triangle_indices {
            self.triangle_indices
                .push(other_triangle_index + base_triangle_index);
        }
        self.vertices.append(&mut other.vertices);
    }
}

pub struct PolygonMeshBuilder {
    earcutr_inputs: Vec<EarcutrInput>,
}

impl PolygonMeshBuilder {
    pub fn new() -> Self {
        PolygonMeshBuilder {
            earcutr_inputs: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_earcutr_input(&mut self, earcutr_input: EarcutrInput) {
        self.earcutr_inputs.push(earcutr_input);
    }

    pub fn build(self) -> Mesh {
        let result = self.run_earcutr();
        build_mesh_from_earcutr(result)
    }

    fn run_earcutr(self) -> EarcutrResult {
        let mut earcutr_inputs_iter = self.earcutr_inputs.into_iter();

        // Earcut the first polygon
        let first_input = match earcutr_inputs_iter.next() {
            Some(i) => i,
            None => {
                return EarcutrResult {
                    triangle_indices: vec![],
                    vertices: vec![],
                }
            }
        };
        let first_triangle_indices =
            earcutr::earcut(&first_input.vertices, &first_input.interior_indices, 2);
        let mut earcutr_result = EarcutrResult {
            triangle_indices: first_triangle_indices,
            vertices: first_input.vertices,
        };

        // Earcut any additional polygons and merge the results into the result of the first polygon
        for earcutr_input in earcutr_inputs_iter {
            let EarcutrInput {
                vertices,
                interior_indices,
            } = earcutr_input;
            let next_earcutr_result = earcutr::earcut(&vertices, &interior_indices, 2);
            earcutr_result.merge(EarcutrResult {
                triangle_indices: next_earcutr_result,
                vertices: vertices,
            });
        }

        earcutr_result
    }
}

pub fn build_mesh_from_earcutr(earcutr_result: EarcutrResult) -> Mesh {
    let indices = earcutr_result
        .triangle_indices
        .into_iter()
        .map(|n| u32::try_from(n).unwrap())
        .collect::<Vec<_>>();
    let vertices = earcutr_result
        .vertices
        .chunks(2)
        .map(|n| [n[0] as f32, n[1] as f32])
        .collect::<Vec<_>>();
    build_mesh_from_bevy(indices, vertices)
}

fn build_mesh_from_bevy(triangle_indices: BevyIndices, vertices: BevyVertices) -> Mesh {
    let num_vertices = vertices.len();
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(triangle_indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let mut normals = Vec::new();
    normals.resize(num_vertices, [0.0, 0.0, 0.0]);
    let mut uvs = Vec::new();
    uvs.resize(num_vertices, [0.0, 0.0]);

    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}
