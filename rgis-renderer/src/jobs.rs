use geo_bevy::BuildBevyMeshes;

pub struct MeshBuildingJob {
    pub layer_id: rgis_layer_id::LayerId,
    pub color: bevy::render::color::Color,
    pub geometry: geo_projected::Projected<geo::Geometry>,
    pub is_selected: bool,
}

pub struct MeshBuildingJobOutcome {
    pub prepared_meshes: Vec<geo_bevy::PreparedMesh>,
    pub layer_id: rgis_layer_id::LayerId,
    pub is_selected: bool,
}

impl bevy_jobs::Job for MeshBuildingJob {
    type Outcome =
        Result<MeshBuildingJobOutcome, <geo::Geometry as geo_bevy::BuildBevyMeshes>::Error>;

    fn name(&self) -> String {
        "Building Bevy meshes".to_string()
    }

    fn perform(self, _: bevy_jobs::Context) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(MeshBuildingJobOutcome {
                prepared_meshes: self.geometry.as_raw().build_bevy_meshes(self.color)?,
                layer_id: self.layer_id,
                is_selected: self.is_selected,
            })
        })
    }
}
