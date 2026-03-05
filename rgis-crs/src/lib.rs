use bevy::prelude::*;
use rgis_primitives::Crs;

static DEFAULT_TARGET_CRS: u16 = 3857;

#[derive(Resource, Clone)]
pub struct TargetCrs(pub Crs);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, handle_crs_changed_events)
            .add_systems(Startup, insert_target_crs);
    }
}

fn insert_target_crs(
    mut commands: Commands,
    geodesy_ctx: Res<rgis_geodesy::GeodesyContext>,
) -> Result {
    let mut geodesy_ctx = match geodesy_ctx.0.write() {
        Ok(ctx) => ctx,
        Err(_) => {
            return Err("Failed to acquire geodesy context write lock".into());
        }
    };
    let op_handle =
        rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, DEFAULT_TARGET_CRS)?;
    commands.insert_resource(TargetCrs(Crs {
        epsg_code: Some(DEFAULT_TARGET_CRS),
        proj_string: None,
        op_handle,
    }));
    Ok(())
}

fn handle_crs_changed_events(
    mut change_crs_event_reader: MessageReader<rgis_crs_messages::ChangeCrsMessage>,
    mut commands: Commands,
    mut target_crs: ResMut<TargetCrs>,
) {
    if let Some(event) = change_crs_event_reader.read().last() {
        target_crs.0 = event.new.clone();
        commands.trigger(rgis_crs_messages::CrsChangedEvent {
            old: event.old.clone(),
            new: event.new.clone(),
        });
    }
}
