use bevy::prelude::*;

pub struct RgisUi;

// A unit struct to help identify the Position Text component, since there may be many Text components
pub struct PositionText;

impl Plugin for RgisUi {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(text_update_system);
    }
}

fn text_update_system(mut query: Query<&mut Text, With<PositionText>>) {
    for mut text in query.iter_mut() {
        // if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        //     if let Some(average) = fps.average() {
        //         text.value = format!("FPS: {:.2}", average);
        //     }
        // }
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(PositionText);
}
