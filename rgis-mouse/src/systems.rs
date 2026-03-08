use bevy::{
    input::mouse::{MouseButton, MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    window::{PrimaryWindow, SystemCursorIcon},
};

fn run_if_has_cursor_moved_events(cursor_moved_event_reader: MessageReader<CursorMoved>) -> bool {
    !cursor_moved_event_reader.is_empty()
}

fn cursor_moved_system(
    mut cursor_moved_event_reader: MessageReader<CursorMoved>,
    windows: Query<&Window, With<PrimaryWindow>>,
    query: Query<&Transform, With<Camera>>,
    mut mouse_position: ResMut<crate::MousePos>,
    mut last_cursor_screen_position: ResMut<crate::LastCursorScreenPosition>,
    mut bevy_egui_ctx: bevy_egui::EguiContexts,
    map_pane_rect: Res<rgis_units::MapPaneRect>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let pointer_outside_map = bevy_egui_ctx_mut
        .input(|i| i.pointer.hover_pos())
        .map(|pos| !map_pane_rect.contains(pos.x, pos.y))
        .unwrap_or(true);
    if pointer_outside_map {
        cursor_moved_event_reader.clear();
        return Ok(());
    }
    let window = windows.single()?;
    let transform = query.single()?;
    if let Some(event) = cursor_moved_event_reader.read().last() {
        let screen_coord = rgis_units::ScreenCoord {
            x: f64::from(event.position.x),
            y: f64::from(event.position.y),
        };
        mouse_position.0 = screen_coord.to_projected_geo_coord(transform, &window);
        last_cursor_screen_position.0 = Some(screen_coord);
    }
    Ok(())
}

fn run_if_has_mouse_motion_events(mouse_motion_event_reader: MessageReader<MouseMotion>) -> bool {
    !mouse_motion_event_reader.is_empty()
}

// FIXME: Cursor icon setting isn't working
fn mouse_motion_system(
    mut mouse_motion_event_reader: MessageReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut pan_camera_events: MessageWriter<rgis_events::PanCameraMessage>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut bevy_egui_ctx: bevy_egui::EguiContexts,
    current_tool: Res<State<rgis_settings::Tool>>,
    mut last_cursor_icon: Local<Option<SystemCursorIcon>>,
    map_pane_rect: Res<rgis_units::MapPaneRect>,
) -> Result {
    let mut window = windows.single_mut()?;
    if let Some(_cursor_icon) = *last_cursor_icon {
        // FIXME
        // window.cursor.icon = cursor_icon;
    }

    // If the pointer is outside the map pane, release input to egui
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let pointer_outside_map = bevy_egui_ctx_mut
        .input(|i| i.pointer.hover_pos())
        .map(|pos| !map_pane_rect.contains(pos.x, pos.y))
        .unwrap_or(true);
    if pointer_outside_map {
        mouse_motion_event_reader.clear();
        clear_cursor_icon(&mut last_cursor_icon);
        return Ok(());
    }

    // Handle panning
    if *current_tool.get() == rgis_settings::Tool::Pan
        && mouse_button.pressed(MouseButton::Left)
        || mouse_button.pressed(MouseButton::Right)
    {
        set_cursor_icon(
            &mut window,
            &mut last_cursor_icon,
            SystemCursorIcon::Grabbing,
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
            pan_camera_events.write(rgis_events::PanCameraMessage { x: x_sum, y: y_sum });
        }
        return Ok(());
    }

    mouse_motion_event_reader.clear();
    let cursor_icon = match *current_tool.get() {
        rgis_settings::Tool::Pan => SystemCursorIcon::Grab,
        rgis_settings::Tool::Query => SystemCursorIcon::Crosshair,
        rgis_settings::Tool::Measure => SystemCursorIcon::Crosshair,
    };
    set_cursor_icon(&mut window, &mut last_cursor_icon, cursor_icon);

    Ok(())
}

fn set_cursor_icon(
    _window: &mut Window,
    last_cursor_icon: &mut Option<SystemCursorIcon>,
    cursor_icon: SystemCursorIcon,
) {
    *last_cursor_icon = Some(cursor_icon);
    // FIXME
    // window.icon = cursor_icon;
}

fn clear_cursor_icon(last_cursor_icon: &mut Option<SystemCursorIcon>) {
    *last_cursor_icon = None;
}

const DRAG_HIT_RADIUS_PX: f64 = 10.0;

fn measure_input_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut measure_state: ResMut<crate::MeasureState>,
    mouse_position: Res<crate::MousePos>,
    last_cursor_screen_pos: Res<crate::LastCursorScreenPosition>,
    camera_q: Query<&Transform, With<Camera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) -> Result {
    if mouse_button.just_pressed(MouseButton::Left) {
        if measure_state.start.is_none() {
            // No points yet: set start
            measure_state.start = Some(mouse_position.0);
        } else if measure_state.end.is_none() {
            // Start set, no end: set end
            measure_state.end = Some(mouse_position.0);
        } else {
            // Both set: hit-test endpoints for dragging
            if let Some(cursor_screen) = last_cursor_screen_pos.0 {
                let transform = camera_q.single()?;
                let window = windows.single()?;

                let start = measure_state.start.unwrap();
                let end = measure_state.end.unwrap();

                let start_screen = rgis_units::ScreenCoord::from_projected(
                    geo::Coord { x: start.x.0, y: start.y.0 },
                    transform,
                    window,
                );
                let end_screen = rgis_units::ScreenCoord::from_projected(
                    geo::Coord { x: end.x.0, y: end.y.0 },
                    transform,
                    window,
                );

                let dist_to_start = ((cursor_screen.x - start_screen.x).powi(2)
                    + (cursor_screen.y - start_screen.y).powi(2))
                .sqrt();
                let dist_to_end = ((cursor_screen.x - end_screen.x).powi(2)
                    + (cursor_screen.y - end_screen.y).powi(2))
                .sqrt();

                if dist_to_start <= DRAG_HIT_RADIUS_PX && dist_to_start <= dist_to_end {
                    measure_state.dragging = Some(crate::MeasureDragTarget::Start);
                } else if dist_to_end <= DRAG_HIT_RADIUS_PX {
                    measure_state.dragging = Some(crate::MeasureDragTarget::End);
                }
            }
        }
    }

    // While dragging, update the dragged point
    if mouse_button.pressed(MouseButton::Left) {
        if let Some(target) = measure_state.dragging {
            match target {
                crate::MeasureDragTarget::Start => {
                    measure_state.start = Some(mouse_position.0);
                }
                crate::MeasureDragTarget::End => {
                    measure_state.end = Some(mouse_position.0);
                }
            }
        }
    }

    // Release clears dragging
    if mouse_button.just_released(MouseButton::Left) {
        measure_state.dragging = None;
    }

    Ok(())
}

fn run_if_has_mouse_scroll_events(mouse_scroll_event_reader: MessageReader<MouseWheel>) -> bool {
    !mouse_scroll_event_reader.is_empty()
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: MessageReader<MouseWheel>,
    mut zoom_camera_events: MessageWriter<rgis_events::ZoomCameraMessage>,
    mouse_position: Res<crate::MousePos>,
    mut bevy_egui_ctx: bevy_egui::EguiContexts,
    map_pane_rect: Res<rgis_units::MapPaneRect>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let pointer_outside_map = bevy_egui_ctx_mut
        .input(|i| i.pointer.hover_pos())
        .map(|pos| !map_pane_rect.contains(pos.x, pos.y))
        .unwrap_or(true);
    if pointer_outside_map {
        return Ok(());
    }

    let y_amount = mouse_scroll_event_reader
        .read()
        .map(|event| {
            if let MouseScrollUnit::Line = event.unit {
                // Magic number was chosen because it resulted in a reasonable scrolling velocity
                // with a mouse on macOS.
                event.y * 10.
            } else {
                event.y
            }
        })
        .sum();
    if y_amount != 0. {
        zoom_camera_events.write(rgis_events::ZoomCameraMessage::new(
            y_amount,
            mouse_position.0,
        ));
    }
    Ok(())
}

fn run_if_has_recalculate_mouse_position_events(
    recalculate_mouse_position_event_reader: MessageReader<
        rgis_events::RecalculateMousePositionMessage,
    >,
) -> bool {
    !recalculate_mouse_position_event_reader.is_empty()
}

fn recalculate_mouse_position_system(
    mut recalculate_mouse_position_event_reader: MessageReader<
        rgis_events::RecalculateMousePositionMessage,
    >,
    mut mouse_position: ResMut<crate::MousePos>,
    last_cursor_screen_position: Res<crate::LastCursorScreenPosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
    query: Query<&Transform, With<Camera>>,
) -> Result {
    recalculate_mouse_position_event_reader.clear();

    let window = windows.single()?;
    let transform = query.single()?;

    if let Some(last_cursor_screen_position) = last_cursor_screen_position.0 {
        mouse_position.0 = last_cursor_screen_position.to_projected_geo_coord(transform, window);
    }

    Ok(())
}

pub fn configure(app: &mut App) {
    // https://github.com/vladbat00/bevy_egui/issues/47#issuecomment-2368811068
    app.add_systems(
        PreUpdate,
        (
            cursor_moved_system.run_if(run_if_has_cursor_moved_events),
            recalculate_mouse_position_system.run_if(run_if_has_recalculate_mouse_position_events),
            mouse_scroll_system.run_if(run_if_has_mouse_scroll_events),
            measure_input_system.run_if(in_state(rgis_settings::Tool::Measure)),
            mouse_motion_system.run_if(run_if_has_mouse_motion_events),
        )
            .after(bevy_egui::EguiPreUpdateSet::ProcessInput)
            .before(bevy_egui::EguiPreUpdateSet::BeginPass),
    );
}
