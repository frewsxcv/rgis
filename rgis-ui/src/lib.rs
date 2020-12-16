use bevy::prelude::*;

pub struct RgisUi;

// A unit struct to help identify the Position Text component, since there may be many Text components
pub struct PositionText;

impl Plugin for RgisUi {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.), Val::Auto),
                margin: Rect::all(Val::Px(10.)),
                ..Default::default()
            },
            material: materials.add(Color::rgba_u8(0, 0, 0, 200).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        margin: Rect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "Lng:\nLat:".to_string(),
                        font: asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf"),
                        style: TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(PositionText);
        });
}
