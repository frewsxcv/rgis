pub struct MeshBuildingJob {
    pub layer_id: rgis_primitives::LayerId,
    pub geometry: geo::Geometry<geo_projected::ProjectedScalar>,
    pub is_selected: bool,
}

pub struct MeshBuildingJobOutcome {
    pub geometry_mesh: geo_bevy::GeometryMesh,
    pub layer_id: rgis_primitives::LayerId,
    pub is_selected: bool,
}

impl bevy_jobs::Job for MeshBuildingJob {
    type Outcome = Result<MeshBuildingJobOutcome, geo_bevy::Error>;

    fn name(&self) -> String {
        "Building Bevy meshes".to_string()
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        let geometry_mesh = geo_bevy::geometry_to_mesh(&self.geometry)?;
        Ok(MeshBuildingJobOutcome {
            geometry_mesh,
            layer_id: self.layer_id,
            is_selected: self.is_selected,
        })
    }
}
