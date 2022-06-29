pub struct MeshBuildingTask {
    pub layer_id: rgis_layer_id::LayerId,
    pub color: bevy::render::color::Color,
    pub geometry: geo::Geometry,
}

impl rgis_task::Task for MeshBuildingTask {
    type Outcome = Result<
        (Vec<geo_bevy::PreparedMesh>, rgis_layer_id::LayerId),
        <geo::Geometry as geo_bevy::BuildBevyMeshes>::Error,
    >;

    fn name(&self) -> String {
        "Building Bevy meshes".to_string()
    }

    fn perform(self) -> rgis_task::PerformReturn<Self::Outcome> {
        Box::pin(async move {
            Ok((
                geo_bevy::build_bevy_meshes(
                    &self.geometry,
                    self.color,
                    geo_bevy::BuildBevyMeshesContext::new(),
                )?
                .collect::<Vec<_>>(),
                self.layer_id,
            ))
        })
    }
}
