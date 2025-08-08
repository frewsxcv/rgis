pub struct MeshBuildingJob {
    pub layer_id: rgis_primitives::LayerId,
    pub geometry: geo::Geometry<geo_projected::ProjectedScalar>,
    pub is_selected: bool,
}

use crate::mesh_conversion;

pub struct MeshBuildingJobOutcome {
    pub geometry_mesh: mesh_conversion::GeometryMesh,
    pub layer_id: rgis_primitives::LayerId,
    pub is_selected: bool,
}

impl bevy_jobs::Job for MeshBuildingJob {
    type Outcome = Result<MeshBuildingJobOutcome, mesh_conversion::Error>;

    fn name(&self) -> String {
        "Building Bevy meshes".to_string()
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        let geometry_mesh = mesh_conversion::geometry_to_mesh(&self.geometry)?;
        Ok(MeshBuildingJobOutcome {
            geometry_mesh,
            layer_id: self.layer_id,
            is_selected: self.is_selected,
        })
    }
}
