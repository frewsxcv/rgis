use bevy::prelude::*;

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
        cursor_moved_event_reader.clear();
        return;
    }
    let window = windows.primary();
    let transform = query.single();
    if let Some(event) = cursor_moved_event_reader.iter().next_back() {
        mouse_position.0 = rgis_units::ScreenCoord {
            x: f64::from(event.position.x),
            y: f64::from(event.position.y),
        }
        .to_projected_geo_coord(transform, window);
    }
}

fn mouse_motion_system(
    mut mouse_motion_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseMotion>,
    mouse_button: Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: bevy::ecs::event::EventWriter<rgis_events::PanCameraEvent>,
    mut windows: ResMut<bevy::window::Windows>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    mut last_cursor_icon: Local<Option<bevy::window::CursorIcon>>,
) {
    if let Some(cursor_icon) = *last_cursor_icon {
        windows.primary_mut().set_cursor_icon(cursor_icon);
    }

    // If the mouse didn't move, then don't go any further
    if mouse_motion_event_reader.is_empty() {
        return;
    }

    // If egui wants to do something with the mouse then release the cursor icon to it
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut();
    if bevy_egui_ctx_mut.wants_pointer_input()
        || bevy_egui_ctx_mut.is_pointer_over_area()
        || bevy_egui_ctx_mut.is_using_pointer()
    {
        mouse_motion_event_reader.clear();
        clear_cursor_icon(&mut last_cursor_icon);
        return;
    }

    // Handle panning
    if rgis_settings.current_tool == rgis_settings::Tool::Pan
        && mouse_button.pressed(bevy::input::mouse::MouseButton::Left)
        || mouse_button.pressed(bevy::input::mouse::MouseButton::Right)
    {
        set_cursor_icon(
            &mut windows,
            &mut last_cursor_icon,
            bevy::window::CursorIcon::Grabbing,
        );
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

    mouse_motion_event_reader.clear();
    let cursor_icon = match rgis_settings.current_tool {
        rgis_settings::Tool::Pan => bevy::window::CursorIcon::Grab,
        rgis_settings::Tool::Query => bevy::window::CursorIcon::Crosshair,
    };
    set_cursor_icon(&mut windows, &mut last_cursor_icon, cursor_icon);
}

fn set_cursor_icon(
    windows: &mut bevy::window::Windows,
    last_cursor_icon: &mut Option<bevy::window::CursorIcon>,
    cursor_icon: bevy::window::CursorIcon,
) {
    *last_cursor_icon = Some(cursor_icon);
    windows.primary_mut().set_cursor_icon(cursor_icon);
}

fn clear_cursor_icon(last_cursor_icon: &mut Option<bevy::window::CursorIcon>) {
    *last_cursor_icon = None;
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
        map_clicked_event_writer.send(rgis_events::MapClickedEvent(mouse_position.0))
    }
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseWheel>,
    mut zoom_camera_events: bevy::ecs::event::EventWriter<rgis_events::ZoomCameraEvent>,
    mouse_position: Res<crate::MousePos>,
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
        zoom_camera_events.send(rgis_events::ZoomCameraEvent::new(
            y_amount,
            mouse_position.0,
        ));
    }
}

pub fn system_set() -> bevy::ecs::schedule::SystemSet {
    bevy::ecs::schedule::SystemSet::new()
        .with_system(cursor_moved_system)
        .with_system(mouse_scroll_system)
        .with_system(mouse_click_system)
        .with_system(mouse_motion_system.after("rgis_ui")) // Egui mouseover functions are dependent on the UI finished setting up
}
