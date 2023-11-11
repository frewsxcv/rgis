pub fn handle_crs_changed_events(
    mut change_crs_event_reader: bevy::ecs::event::EventReader<rgis_events::ChangeCrsEvent>,
    mut crs_changed_event_writer: bevy::ecs::event::EventWriter<rgis_events::CrsChangedEvent>,
    mut settings: bevy::ecs::system::ResMut<crate::RgisSettings>,
) {
    if let Some(event) = change_crs_event_reader.iter().last() {
        settings.target_crs_epsg_code = event.new_crs_epsg_code;
        crs_changed_event_writer.send(rgis_events::CrsChangedEvent {
            old_crs_epsg_code: event.old_crs_epsg_code,
            new_crs_epsg_code: event.new_crs_epsg_code,
        });
    }
}
