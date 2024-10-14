use bevy::prelude::*;

fn handle_toggle_layer_visibility_events(
    mut toggle_layer_visibility_event_reader: EventReader<rgis_events::ToggleLayerVisibilityEvent>,
    mut layer_became_visible_event_writer: EventWriter<rgis_events::LayerBecameVisibleEvent>,
    mut layer_became_hidden_event_writer: EventWriter<rgis_events::LayerBecameHiddenEvent>,
    mut layers: ResMut<crate::Layers>,
) {
    for event in toggle_layer_visibility_event_reader.read() {
        let Some(layer) = layers.get_mut(event.0) else {
            bevy::log::warn!("Could not find layer");
            continue;
        };
        layer.visible = !layer.visible;
        if layer.visible {
            layer_became_visible_event_writer.send(rgis_events::LayerBecameVisibleEvent(event.0));
        } else {
            layer_became_hidden_event_writer.send(rgis_events::LayerBecameHiddenEvent(event.0));
        }
    }
}

fn handle_update_color_events(
    mut update_events: EventReader<rgis_events::UpdateLayerColorEvent>,
    mut updated_events: EventWriter<rgis_events::LayerColorUpdatedEvent>,
    mut layers: ResMut<crate::Layers>,
) {
    for event in update_events.read() {
        let event = match event {
            rgis_events::UpdateLayerColorEvent::Stroke(layer_id, color) => {
                let Some(layer) = layers.get_mut(*layer_id) else {
                    bevy::log::warn!("Could not find layer");
                    continue;
                };
                layer.color.stroke = *color;
                rgis_events::LayerColorUpdatedEvent::Stroke(*layer_id)
            }
            rgis_events::UpdateLayerColorEvent::Fill(layer_id, color) => {
                let Some(layer) = layers.get_mut(*layer_id) else {
                    bevy::log::warn!("Could not find layer");
                    continue;
                };
                layer.color.fill = Some(*color);
                rgis_events::LayerColorUpdatedEvent::Fill(*layer_id)
            }
        };
        updated_events.send(event);
    }
}

fn handle_delete_layer_events(
    mut delete_layer_event_reader: EventReader<rgis_events::DeleteLayerEvent>,
    mut despawn_meshes_event_writer: EventWriter<rgis_events::DespawnMeshesEvent>,
    mut layers: ResMut<crate::Layers>,
) {
    for event in delete_layer_event_reader.read() {
        layers.remove(event.0);
        despawn_meshes_event_writer.send(rgis_events::DespawnMeshesEvent(event.0));
    }
}

fn handle_move_layer_events(
    mut move_layer_event_reader: EventReader<rgis_events::MoveLayerEvent>,
    mut layer_z_index_updated_event_writer: EventWriter<rgis_events::LayerZIndexUpdatedEvent>,
    mut layers: ResMut<crate::Layers>,
) {
    for event in move_layer_event_reader.read() {
        let old_z_index = match layers.get_with_index(event.0) {
            Some(result) => result.1 .0,
            None => {
                bevy::log::warn!("Could not find layer");
                continue;
            }
        };

        let new_z_index = match event.1 {
            rgis_events::MoveDirection::Up => {
                if old_z_index < layers.count() - 1 {
                    old_z_index + 1
                } else {
                    old_z_index
                }
            }
            rgis_events::MoveDirection::Down => {
                if old_z_index > 0 {
                    old_z_index - 1
                } else {
                    old_z_index
                }
            }
        };
        if new_z_index != old_z_index {
            let Some(other_layer_id) = layers.data.get(new_z_index).map(|l| l.id) else {
                bevy::log::warn!("Could not find layer");
                continue;
            };

            layers.data.swap(old_z_index, new_z_index);

            layer_z_index_updated_event_writer.send(rgis_events::LayerZIndexUpdatedEvent(event.0));
            layer_z_index_updated_event_writer
                .send(rgis_events::LayerZIndexUpdatedEvent(other_layer_id));
        }
    }
}

fn handle_map_clicked_events(
    mut map_clicked_event_reader: EventReader<rgis_events::MapClickedEvent>,
    mut render_message_event_writer: EventWriter<rgis_events::RenderFeaturePropertiesEvent>,
    mut feature_clicked_event_writer: EventWriter<rgis_events::FeatureSelectedEvent>,
    layers: Res<crate::Layers>,
) {
    for event in map_clicked_event_reader.read() {
        if let Some((layer_id, feature)) = layers.feature_from_click(event.0) {
            render_message_event_writer.send(rgis_events::RenderFeaturePropertiesEvent(
                feature.properties.clone(),
            ));
            feature_clicked_event_writer
                .send(rgis_events::FeatureSelectedEvent(layer_id, feature.id));
        }
    }
}

fn handle_create_layer_events(
    mut create_layer_events: ResMut<bevy::ecs::event::Events<rgis_events::CreateLayerEvent>>,
    mut layer_created_event_writer: EventWriter<rgis_events::LayerCreatedEvent>,
    mut layers: ResMut<crate::Layers>,
) {
    for event in create_layer_events.drain() {
        let layer_id = layers.add(
            event.feature_collection,
            event.name,
            event.source_crs_epsg_code,
        );
        layer_created_event_writer.send(rgis_events::LayerCreatedEvent(layer_id));
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_toggle_layer_visibility_events,
            handle_update_color_events,
            handle_move_layer_events,
            handle_delete_layer_events,
            handle_map_clicked_events,
            handle_create_layer_events,
        ),
    );
}
