use bevy::prelude::*;
use rgis_primitives::Crs;

static DEFAULT_TARGET_CRS: u16 = 3857;

#[derive(Resource, Clone, Copy)]
pub struct TargetCrs(pub Crs);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(TargetCrs(Crs {
            epsg_code: DEFAULT_TARGET_CRS,
            op_handle: geodesy::OpHandle::default(), // FIXME
        }))
        .add_systems(Update, handle_crs_changed_events);
    }
}

fn handle_crs_changed_events(
    mut change_crs_event_reader: EventReader<rgis_events::ChangeCrsEvent>,
    mut crs_changed_event_writer: EventWriter<rgis_events::CrsChangedEvent>,
    mut target_crs: ResMut<TargetCrs>,
) {
    if let Some(event) = change_crs_event_reader.read().last() {
        target_crs.0 = event.new;
        crs_changed_event_writer.write(rgis_events::CrsChangedEvent {
            old: event.old,
            new: event.new,
        });
    }
}
