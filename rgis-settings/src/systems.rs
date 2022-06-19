pub fn handle_crs_changed_events(
    mut change_crs_event_reader: bevy::ecs::event::EventReader<rgis_events::ChangeCrsEvent>,
    mut settings: bevy::ecs::system::ResMut<crate::RgisSettings>,
) {
	for event in change_crs_event_reader.iter() {
        settings.target_crs = event.crs.clone();
    }
}
