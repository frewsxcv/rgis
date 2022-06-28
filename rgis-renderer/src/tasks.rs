pub struct MeshBuildingTask {
    pub layer_id: rgis_layer_id::LayerId,
    pub geometry: geo::Geometry,
}

impl rgis_task::Task for MeshBuildingTask {
    type Outcome = Result<
        (Vec<bevy::render::mesh::Mesh>, rgis_layer_id::LayerId),
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
                    geo_bevy::BuildBevyMeshesContext::new(),
                )?
                .collect::<Vec<bevy::render::mesh::Mesh>>(),
                self.layer_id,
            ))
        })
    }
}
