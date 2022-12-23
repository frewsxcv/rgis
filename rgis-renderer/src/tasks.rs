pub struct MeshBuildingTask {
    pub layer_id: rgis_layer_id::LayerId,
    pub color: bevy::render::color::Color,
    pub geometry: geo_projected::Projected<geo::Geometry>,
    pub is_selected: bool,
}

pub struct MeshBuildingTaskOutcome {
    pub prepared_meshes: Vec<geo_bevy::PreparedMesh>,
    pub layer_id: rgis_layer_id::LayerId,
    pub is_selected: bool,
}

impl bevy_jobs::Job for MeshBuildingTask {
    type Outcome =
        Result<MeshBuildingTaskOutcome, <geo::Geometry as geo_bevy::BuildBevyMeshes>::Error>;

    fn name(&self) -> String {
        "Building Bevy meshes".to_string()
    }

    fn perform(self) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(MeshBuildingTaskOutcome {
                prepared_meshes: geo_bevy::build_bevy_meshes(
                    self.geometry.as_raw(),
                    self.color,
                    geo_bevy::BuildBevyMeshesContext::new(),
                )?
                .collect::<Vec<_>>(),
                layer_id: self.layer_id,
                is_selected: self.is_selected,
            })
        })
    }
}
