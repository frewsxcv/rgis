use crate::mesh_conversion::line_string::LineStringMeshBuilder;
use bevy::prelude::Mesh;
use geo_traits::*;

pub struct PolygonMesh {
    pub mesh: Mesh,
    pub exterior_mesh: Mesh,
    pub interior_meshes: Vec<Mesh>,
}

pub struct PolygonMeshBuilder<Scalar: geo_types::CoordFloat> {
    polygon: bevy_earcutr::PolygonMeshBuilder<Scalar>,
    exterior: LineStringMeshBuilder,
    interiors: Vec<LineStringMeshBuilder>,
}

impl<Scalar: geo_types::CoordFloat> Default for PolygonMeshBuilder<Scalar> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Scalar: geo_types::CoordFloat> PolygonMeshBuilder<Scalar> {
    pub fn new() -> Self {
        Self {
            polygon: bevy_earcutr::PolygonMeshBuilder::default(),
            exterior: LineStringMeshBuilder::default(),
            interiors: Vec::new(),
        }
    }

    pub fn add_polygon(
        &mut self,
        polygon: &impl geo_traits::PolygonTrait<T = Scalar>,
    ) -> Result<(), crate::mesh_conversion::Error> {
        self.polygon
            .add_earcutr_input(Self::polygon_to_earcutr_input(polygon));
        if let Some(exterior) = polygon.exterior() {
            self.exterior.add_coords(exterior.coords())?;
        }
        for interior in polygon.interiors() {
            self.interiors.push(LineStringMeshBuilder::default());
            self.interiors
                .last_mut()
                .unwrap()
                .add_coords(interior.coords())?;
        }
        Ok(())
    }

    pub fn add_polygon_from_exterior_coords(
        &mut self,
        coords: impl Iterator<Item = impl CoordTrait<T = Scalar>> + Clone,
    ) -> Result<(), crate::mesh_conversion::Error> {
        self.polygon
            .add_earcutr_input(Self::exterior_coords_to_earcutr_input(coords.clone()));
        self.exterior.add_coords(coords)?;
        Ok(())
    }

    fn polygon_to_earcutr_input(
        polygon: &impl geo_traits::PolygonTrait<T = Scalar>,
    ) -> bevy_earcutr::EarcutrInput<Scalar> {
        let mut vertices = Vec::with_capacity(polygon_coords_count(polygon) * 2);
        let mut interior_indices = Vec::with_capacity(polygon.num_interiors());
        debug_assert!(
            polygon
                .exterior()
                .map_or(0, |exterior| exterior.num_coords())
                >= 4
        );

        if let Some(exterior) = polygon.exterior() {
            Self::flat_line_string_coords_2(exterior.coords(), &mut vertices);
        }

        for interior in polygon.interiors() {
            debug_assert!(interior.num_coords() >= 4);
            interior_indices.push(vertices.len() / 2);
            Self::flat_line_string_coords_2(interior.coords(), &mut vertices);
        }

        bevy_earcutr::EarcutrInput {
            vertices,
            interior_indices,
        }
    }

    fn exterior_coords_to_earcutr_input(
        exterior: impl Iterator<Item = impl CoordTrait<T = Scalar>> + Clone,
    ) -> bevy_earcutr::EarcutrInput<Scalar> {
        let count = exterior.clone().count();
        let mut vertices = Vec::with_capacity(count * 2);
        debug_assert!(count >= 4);

        Self::flat_line_string_coords_2(exterior, &mut vertices);

        bevy_earcutr::EarcutrInput {
            vertices,
            interior_indices: vec![],
        }
    }

    fn flat_line_string_coords_2(
        line_string_coords: impl Iterator<Item = impl CoordTrait<T = Scalar>>,
        vertices: &mut Vec<Scalar>,
    ) {
        for coord in line_string_coords {
            vertices.push(coord.x());
            vertices.push(coord.y());
        }
    }
}

fn polygon_coords_count<P: PolygonTrait>(polygon: &P) -> usize {
    polygon
        .exterior()
        .map_or(0, |exterior| exterior.num_coords())
        + polygon
            .interiors()
            .map(|interior| interior.num_coords())
            .sum::<usize>()
}

impl<Scalar: geo_types::CoordFloat> TryFrom<PolygonMeshBuilder<Scalar>> for PolygonMesh {
    type Error = crate::mesh_conversion::Error;

    fn try_from(polygon_mesh_builder: PolygonMeshBuilder<Scalar>) -> Result<Self, Self::Error> {
        polygon_mesh_builder
            .polygon
            .build()
            .map_err(crate::mesh_conversion::Error::BevyEarcutr)
            .and_then(|polygon_mesh| {
                let exterior_mesh = Mesh::try_from(polygon_mesh_builder.exterior)?;
                let interior_meshes = polygon_mesh_builder
                    .interiors
                    .into_iter()
                    .map(Mesh::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PolygonMesh {
                    mesh: polygon_mesh,
                    exterior_mesh,
                    interior_meshes,
                })
            })
    }
}

impl<Scalar: geo_types::CoordFloat> crate::mesh_conversion::build_mesh::BuildMesh
    for PolygonMeshBuilder<Scalar>
{
    fn build(self) -> Result<crate::mesh_conversion::GeometryMesh, crate::mesh_conversion::Error> {
        Ok(crate::mesh_conversion::GeometryMesh::Polygon(
            PolygonMesh::try_from(self)?,
        ))
    }
}
