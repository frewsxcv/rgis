use bevy::ecs::system::{Query, Res, ResMut};

fn cursor_moved_system(
    mut cursor_moved_event_reader: bevy::ecs::event::EventReader<bevy::window::CursorMoved>,
    windows: Res<bevy::window::Windows>,
    query: Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<bevy::render::camera::Camera>,
    >,
    mut mouse_position: ResMut<crate::MousePos>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    if cursor_moved_event_reader.is_empty() {
        return;
    }
    if bevy_egui_ctx.ctx_mut().is_pointer_over_area() {
        return;
    }
    let window = windows.primary();
    let transform = query.single();
    if let Some(event) = cursor_moved_event_reader.iter().next_back() {
        mouse_position.projected = screen_coords_to_geo_coords(event.position, transform, window);
    }
}

fn screen_coords_to_geo_coords(
    screen_coords: bevy::prelude::Vec2,
    transform: &bevy::transform::components::Transform,
    window: &bevy::prelude::Window,
) -> geo::Coordinate {
    let size = bevy::math::Vec2::new(window.width(), window.height());

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = screen_coords - size / 2.0;

    // apply the camera transform
    let pos_wld = transform.compute_matrix() * p.extend(0.0).extend(1.0);

    geo::Coordinate {
        x: pos_wld.x.into(),
        y: pos_wld.y.into(),
    }
}

fn mouse_motion_system(
    mut mouse_motion_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseMotion>,
    mouse_button: Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: bevy::ecs::event::EventWriter<rgis_events::PanCameraEvent>,
    mut windows: ResMut<bevy::window::Windows>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
) {
    if bevy_egui_ctx.ctx_mut().is_pointer_over_area() {
        windows
            .primary_mut()
            .set_cursor_icon(bevy::window::CursorIcon::Arrow);
        return;
    }

    if rgis_settings.current_tool == rgis_settings::Tool::Pan
        && mouse_button.pressed(bevy::input::mouse::MouseButton::Left)
        || mouse_button.pressed(bevy::input::mouse::MouseButton::Right)
    {
        windows
            .primary_mut()
            .set_cursor_icon(bevy::window::CursorIcon::Grabbing);
        let mut x_sum = 0.;
        let mut y_sum = 0.;
        for event in mouse_motion_event_reader.iter() {
            // If the mouse is dragging rightward, `delta.x` will be positive. In this case, we
            // want the map to move right, and the camera to move left. We need to negate the
            // delta X value.
            x_sum -= event.delta.x;

            // If the mouse is dragging upward, `delta.y` will be negative. In this case, we
            // want the map to move up, and the camera to move down. We do not need to negate
            // the delta Y value.
            y_sum += event.delta.y;
        }
        if x_sum != 0. || y_sum != 0. {
            pan_camera_events.send(rgis_events::PanCameraEvent { x: x_sum, y: y_sum });
        }
        return;
    }

    let cursor_icon = match rgis_settings.current_tool {
        rgis_settings::Tool::Pan => bevy::window::CursorIcon::Grab,
        rgis_settings::Tool::Query => bevy::window::CursorIcon::Crosshair,
    };
    windows.primary_mut().set_cursor_icon(cursor_icon);
}

fn mouse_click_system(
    mut map_clicked_event_writer: bevy::ecs::event::EventWriter<rgis_events::MapClickedEvent>,
    mouse_button: Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    mouse_position: Res<crate::MousePos>,
) {
    if rgis_settings.current_tool == rgis_settings::Tool::Query
        && mouse_button.just_pressed(bevy::input::mouse::MouseButton::Left)
    {
        map_clicked_event_writer.send(rgis_events::MapClickedEvent(mouse_position.projected))
    }
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseWheel>,
    mut zoom_camera_events: bevy::ecs::event::EventWriter<rgis_events::ZoomCameraEvent>,
) {
    let y_amount = mouse_scroll_event_reader
        .iter()
        .map(|event| {
            if let bevy::input::mouse::MouseScrollUnit::Line = event.unit {
                // Magic number was chosen because it resulted in a reasonable scrolling velocity
                // with a mouse on macOS.
                event.y * 10.
            } else {
                event.y
            }
        })
        .sum();
    if y_amount != 0. {
        zoom_camera_events.send(rgis_events::ZoomCameraEvent::new(y_amount));
    }
}

pub fn system_set() -> bevy::ecs::schedule::SystemSet {
    bevy::ecs::schedule::SystemSet::new()
        .with_system(cursor_moved_system)
        .with_system(mouse_scroll_system)
        .with_system(mouse_click_system)
        .with_system(mouse_motion_system)
}
