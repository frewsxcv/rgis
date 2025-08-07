use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct ShowManageLayerWindowEvent(pub rgis_primitives::LayerId);

#[derive(Default, Event)]
pub struct OpenChangeCrsWindow;

#[derive(Default, Event)]
pub struct ShowAddLayerWindow;

#[derive(Default, Event)]
pub struct HideAddLayerWindow;

#[derive(Event)]
pub struct RenderMessageEvent(pub String);

#[derive(Event)]
pub struct RenderFeaturePropertiesEvent {
    pub layer_id: rgis_primitives::LayerId,
    pub properties: geo_features::Properties,
}

/// Change the `Layer`'s color
#[derive(Event)]
pub enum UpdateLayerColorEvent {
    Fill(rgis_primitives::LayerId, bevy::prelude::Color),
    Stroke(rgis_primitives::LayerId, bevy::prelude::Color),
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowManageLayerWindowEvent>()
            .add_event::<OpenChangeCrsWindow>()
            .add_event::<ShowAddLayerWindow>()
            .add_event::<HideAddLayerWindow>()
            .add_event::<RenderMessageEvent>()
            .add_event::<RenderFeaturePropertiesEvent>()
            .add_event::<UpdateLayerColorEvent>();
    }
}
