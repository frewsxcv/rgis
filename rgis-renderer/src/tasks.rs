pub struct MeshBuildingTask {
    pub layer_id: rgis_layer_id::LayerId,
    pub color: bevy::render::color::Color,
    pub geometry: geo_projected::Projected<geo::Geometry>,
}

impl bevy_jobs::Job for MeshBuildingTask {
    type Outcome = Result<
        (Vec<geo_bevy::PreparedMesh>, rgis_layer_id::LayerId),
        <geo::Geometry as geo_bevy::BuildBevyMeshes>::Error,
    >;

    fn name(&self) -> String {
        "Building Bevy meshes".to_string()
    }

    fn perform(self) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            Ok((
                geo_bevy::build_bevy_meshes(
                    self.geometry.as_raw(),
                    self.color,
                    geo_bevy::BuildBevyMeshesContext::new(),
                )?
                .collect::<Vec<_>>(),
                self.layer_id,
            ))
        })
    }
}
