use bevy::{prelude::*, window::PrimaryWindow};

fn run_if_has_cursor_moved_events(
    cursor_moved_event_reader: bevy::ecs::event::EventReader<bevy::window::CursorMoved>,
) -> bool {
    !cursor_moved_event_reader.is_empty()
}

fn cursor_moved_system(
    mut cursor_moved_event_reader: bevy::ecs::event::EventReader<bevy::window::CursorMoved>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    query: Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<bevy::render::camera::Camera>,
    >,
    mut mouse_position: ResMut<crate::MousePos>,
    mut bevy_egui_ctx: bevy_egui::EguiContexts,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    if bevy_egui_ctx_mut.is_pointer_over_area() {
        cursor_moved_event_reader.clear();
        return Ok(());
    }
    let window = windows.single_mut()?;
    let transform = query.single()?;
    if let Some(event) = cursor_moved_event_reader.read().last() {
        mouse_position.0 = rgis_units::ScreenCoord {
            x: f64::from(event.position.x),
            y: f64::from(event.position.y),
        }
        .to_projected_geo_coord(transform, &window);
    }
    Ok(())
}

fn run_if_has_mouse_motion_events(
    mouse_motion_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseMotion>,
) -> bool {
    !mouse_motion_event_reader.is_empty()
}

// FIXME: Cursor icon setting isn't working
fn mouse_motion_system(
    mut mouse_motion_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseMotion>,
    mouse_button: Res<bevy::input::ButtonInput<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: bevy::ecs::event::EventWriter<rgis_events::PanCameraEvent>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut bevy_egui_ctx: bevy_egui::EguiContexts,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    mut last_cursor_icon: Local<Option<bevy::window::SystemCursorIcon>>,
) -> Result {
    let mut window = windows.single_mut()?;
    if let Some(_cursor_icon) = *last_cursor_icon {
        // FIXME
        // window.cursor.icon = cursor_icon;
    }

    // If egui wants to do something with the mouse then release the cursor icon to it
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    if bevy_egui_ctx_mut.wants_pointer_input()
        || bevy_egui_ctx_mut.is_pointer_over_area()
        || bevy_egui_ctx_mut.is_using_pointer()
    {
        mouse_motion_event_reader.clear();
        clear_cursor_icon(&mut last_cursor_icon);
        return Ok(());
    }

    // Handle panning
    if rgis_settings.current_tool == rgis_settings::Tool::Pan
        && mouse_button.pressed(bevy::input::mouse::MouseButton::Left)
        || mouse_button.pressed(bevy::input::mouse::MouseButton::Right)
    {
        set_cursor_icon(
            &mut window,
            &mut last_cursor_icon,
            bevy::window::SystemCursorIcon::Grabbing,
        );
        let mut x_sum = 0.;
        let mut y_sum = 0.;
        for event in mouse_motion_event_reader.read() {
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
            pan_camera_events.write(rgis_events::PanCameraEvent { x: x_sum, y: y_sum });
        }
        return Ok(());
    }

    mouse_motion_event_reader.clear();
    let cursor_icon = match rgis_settings.current_tool {
        rgis_settings::Tool::Pan => bevy::window::SystemCursorIcon::Grab,
        rgis_settings::Tool::Query => bevy::window::SystemCursorIcon::Crosshair,
    };
    set_cursor_icon(&mut window, &mut last_cursor_icon, cursor_icon);

    Ok(())
}

fn set_cursor_icon(
    _window: &mut Window,
    last_cursor_icon: &mut Option<bevy::window::SystemCursorIcon>,
    cursor_icon: bevy::window::SystemCursorIcon,
) {
    *last_cursor_icon = Some(cursor_icon);
    // FIXME
    // window.icon = cursor_icon;
}

fn clear_cursor_icon(last_cursor_icon: &mut Option<bevy::window::SystemCursorIcon>) {
    *last_cursor_icon = None;
}

fn run_if_mouse_left_button_just_pressed(
    mouse_button: Res<bevy::input::ButtonInput<bevy::input::mouse::MouseButton>>,
) -> bool {
    mouse_button.just_pressed(bevy::input::mouse::MouseButton::Left)
}

fn current_tool_is_query(rgis_settings: Res<rgis_settings::RgisSettings>) -> bool {
    rgis_settings.current_tool == rgis_settings::Tool::Query
}

fn mouse_click_system(
    mut map_clicked_event_writer: bevy::ecs::event::EventWriter<rgis_events::MapClickedEvent>,
    mouse_position: Res<crate::MousePos>,
) {
    map_clicked_event_writer.write(rgis_events::MapClickedEvent(mouse_position.0));
}

fn run_if_has_mouse_scroll_events(
    mouse_scroll_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseWheel>,
) -> bool {
    !mouse_scroll_event_reader.is_empty()
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseWheel>,
    mut zoom_camera_events: bevy::ecs::event::EventWriter<rgis_events::ZoomCameraEvent>,
    mouse_position: Res<crate::MousePos>,
    mut bevy_egui_ctx: bevy_egui::EguiContexts,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    if bevy_egui_ctx_mut.wants_pointer_input()
        || bevy_egui_ctx_mut.is_pointer_over_area()
        || bevy_egui_ctx_mut.is_using_pointer()
    {
        return Ok(());
    }

    let y_amount = mouse_scroll_event_reader
        .read()
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
        zoom_camera_events.write(rgis_events::ZoomCameraEvent::new(
            y_amount,
            mouse_position.0,
        ));
    }
    Ok(())
}

pub fn configure(app: &mut App) {
    // https://github.com/vladbat00/bevy_egui/issues/47#issuecomment-2368811068
    app.add_systems(
        PreUpdate,
        (
            cursor_moved_system.run_if(run_if_has_cursor_moved_events),
            mouse_scroll_system.run_if(run_if_has_mouse_scroll_events),
            mouse_click_system
                .run_if(current_tool_is_query)
                .run_if(run_if_mouse_left_button_just_pressed),
            mouse_motion_system.run_if(run_if_has_mouse_motion_events),
        )
            .after(bevy_egui::EguiPreUpdateSet::ProcessInput)
            .before(bevy_egui::EguiPreUpdateSet::BeginPass),
    );
}
