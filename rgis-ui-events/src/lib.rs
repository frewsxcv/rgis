use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct ShowManageLayerWindowEvent(pub rgis_primitives::LayerId);

#[derive(Default, Message)]
pub struct OpenChangeCrsWindow;

#[derive(Default, Message)]
pub struct ShowAddLayerWindow;

#[derive(Default, Message)]
pub struct HideAddLayerWindow;

#[derive(Message)]
pub struct RenderMessageEvent(pub String);

#[derive(Message)]
pub struct RenderFeaturePropertiesEvent {
    pub layer_id: rgis_primitives::LayerId,
    pub properties: geo_features::Properties,
}

/// Change the `Layer`'s color
#[derive(Message)]
pub enum UpdateLayerColorEvent {
    Fill(rgis_primitives::LayerId, bevy::prelude::Color),
    Stroke(rgis_primitives::LayerId, bevy::prelude::Color),
}

#[derive(Message, Debug)]
pub struct UpdateLayerPointSizeEvent(pub rgis_primitives::LayerId, pub f32);

#[derive(Message)]
pub struct OpenOperationWindowEvent {
    pub operation: Box<dyn Send + Sync + rgis_geo_ops::Operation>,
    pub feature_collection: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
}

#[derive(Message)]
pub struct PerformOperationEvent {
    pub operation: Box<dyn Send + Sync + rgis_geo_ops::Operation>,
    pub layer_id: rgis_primitives::LayerId,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShowManageLayerWindowEvent>()
            .add_message::<OpenChangeCrsWindow>()
            .add_message::<ShowAddLayerWindow>()
            .add_message::<HideAddLayerWindow>()
            .add_message::<RenderMessageEvent>()
            .add_message::<RenderFeaturePropertiesEvent>()
            .add_message::<UpdateLayerColorEvent>()
            .add_message::<UpdateLayerPointSizeEvent>()
            .add_message::<OpenOperationWindowEvent>()
            .add_message::<PerformOperationEvent>();
    }
}
