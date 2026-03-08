/// Regression test for a bug where `handle_features_deselected_event` never
/// consumed its `FeaturesDeselectedMessage`, causing the deselection logic to
/// fire every frame indefinitely. On desktop (multi-threaded), this blocked
/// layer loading from completing.
///
/// The fix: use `event_reader.read()` to consume messages instead of only
/// checking `is_empty()`.
use bevy::prelude::*;

/// Mirrors the query filter from `SelectedFeatureQuery` in systems.rs.
type SelectedFeatureQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    (Entity, &'a rgis_renderer::RenderEntityType),
    Or<(With<MeshMaterial2d<ColorMaterial>>, With<Sprite>)>,
>;

/// Minimal system that replicates the fixed behavior: consume messages and
/// despawn selected entities. This must match the logic in
/// `handle_features_deselected_event` in systems.rs.
fn handle_features_deselected_event(
    mut event_reader: MessageReader<rgis_events::FeaturesDeselectedMessage>,
    mut commands: Commands,
    query: SelectedFeatureQuery,
) {
    if event_reader.read().next().is_none() {
        return;
    }
    for (entity, entity_type) in query.iter() {
        match entity_type {
            rgis_renderer::RenderEntityType::SelectedPolygon
            | rgis_renderer::RenderEntityType::SelectedLineString
            | rgis_renderer::RenderEntityType::SelectedPoint => {
                commands.entity(entity).despawn();
            }
            _ => (),
        }
    }
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_message::<rgis_events::FeaturesDeselectedMessage>();
    app.add_systems(Update, handle_features_deselected_event);
    app
}

/// After sending a FeaturesDeselectedMessage, the deselection logic must only
/// fire once. Entities spawned after the message is consumed must NOT be
/// despawned on subsequent frames.
///
/// Before the fix, the system used `is_empty()` without consuming messages,
/// so the deselection logic would fire every frame forever after the first
/// message, despawning any newly-spawned selected entities and interfering
/// with layer loading.
#[test]
fn features_deselected_message_does_not_refire_on_subsequent_frames() {
    let mut app = test_app();
    app.update();

    // Spawn a selected entity and send the deselection message.
    let first_entity = app
        .world_mut()
        .spawn((
            rgis_renderer::RenderEntityType::SelectedPolygon,
            Sprite::default(),
        ))
        .id();

    app.world_mut()
        .resource_mut::<Messages<rgis_events::FeaturesDeselectedMessage>>()
        .write(rgis_events::FeaturesDeselectedMessage);

    // First update: consumes the message and despawns the selected entity.
    app.update();

    assert!(
        app.world().get_entity(first_entity).is_err(),
        "Selected entity should be despawned after FeaturesDeselectedMessage"
    );

    // Now spawn a NEW selected entity WITHOUT sending another message.
    let second_entity = app
        .world_mut()
        .spawn((
            rgis_renderer::RenderEntityType::SelectedPolygon,
            Sprite::default(),
        ))
        .id();

    // Run several more updates — the system must be a no-op since the message
    // was already consumed.
    for _ in 0..5 {
        app.update();
    }

    assert!(
        app.world().get_entity(second_entity).is_ok(),
        "Entity spawned AFTER message was consumed should NOT be despawned — \
         this means the system is re-firing every frame (the original bug)"
    );
}

/// Verify that selected feature entities are despawned when a
/// FeaturesDeselectedMessage is sent, and that non-selected render entities
/// remain untouched.
#[test]
fn features_deselected_despawns_only_selected_entities() {
    let mut app = test_app();
    app.update();

    // Spawn a "selected" entity that the deselection system should remove.
    let selected_entity = app
        .world_mut()
        .spawn((
            rgis_renderer::RenderEntityType::SelectedPolygon,
            Sprite::default(),
        ))
        .id();

    // Spawn a normal (non-selected) entity that should survive.
    let normal_entity = app
        .world_mut()
        .spawn((
            rgis_renderer::RenderEntityType::Polygon,
            Sprite::default(),
        ))
        .id();

    app.world_mut()
        .resource_mut::<Messages<rgis_events::FeaturesDeselectedMessage>>()
        .write(rgis_events::FeaturesDeselectedMessage);

    app.update();

    assert!(
        app.world().get_entity(selected_entity).is_err(),
        "Selected entity should have been despawned by FeaturesDeselectedMessage"
    );

    assert!(
        app.world().get_entity(normal_entity).is_ok(),
        "Non-selected entity should NOT be despawned by FeaturesDeselectedMessage"
    );
}
