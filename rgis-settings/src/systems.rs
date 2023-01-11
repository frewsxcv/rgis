pub fn handle_crs_changed_events(
    mut change_crs_event_reader: bevy::ecs::event::EventReader<rgis_events::ChangeCrsEvent>,
    mut crs_changed_event_writer: bevy::ecs::event::EventWriter<rgis_events::CrsChangedEvent>,
    mut settings: bevy::ecs::system::ResMut<crate::RgisSettings>,
) {
    if let Some(event) = change_crs_event_reader.iter().next_back() {
        settings.target_crs = event.new_crs.clone();
        crs_changed_event_writer.send(rgis_events::CrsChangedEvent {
            old_crs: event.old_crs.clone(),
            new_crs: event.new_crs.clone(),
        });
    }
}
