use bevy::prelude::*;
use rgis_primitives::Crs;

#[derive(Resource, Clone)]
pub struct TargetCrs(pub Crs);

/// Configuration for the CRS plugin.
pub struct Plugin {
    /// The default target CRS EPSG code used on startup.
    pub default_crs: u16,
}

impl Default for Plugin {
    fn default() -> Self {
        Self { default_crs: 3857 }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(DefaultTargetCrs(self.default_crs))
            .add_systems(Update, handle_crs_changed_events)
            .add_systems(Startup, insert_target_crs);
    }
}

/// Resource holding the configured default target CRS EPSG code.
#[derive(Resource)]
struct DefaultTargetCrs(u16);

fn insert_target_crs(
    mut commands: Commands,
    geodesy_ctx: Res<rgis_geodesy::GeodesyContext>,
    default_target_crs: Res<DefaultTargetCrs>,
) -> Result {
    let default_crs = default_target_crs.0;
    let mut geodesy_ctx = match geodesy_ctx.write() {
        Ok(ctx) => ctx,
        Err(_) => {
            return Err("Failed to acquire geodesy context write lock".into());
        }
    };
    let op_handle =
        rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, default_crs)?;
    commands.insert_resource(TargetCrs(Crs {
        epsg_code: Some(default_crs),
        proj_string: None,
        op_handle,
    }));
    Ok(())
}

fn handle_crs_changed_events(
    mut change_crs_event_reader: MessageReader<rgis_events::ChangeCrsMessage>,
    mut commands: Commands,
    mut target_crs: ResMut<TargetCrs>,
) {
    if let Some(event) = change_crs_event_reader.read().last() {
        target_crs.0 = event.new.clone();
        commands.trigger(rgis_events::CrsChangedEvent {
            old: event.old.clone(),
            new: event.new.clone(),
        });
    }
}
