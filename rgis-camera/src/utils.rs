use bevy::prelude::*;

pub(crate) fn center_camera_on_projected_world_rect(
    bounding_rect: crate::ProjectedWorldRect,
    camera_transform: &mut Transform,
    window: &bevy::window::Window,
    side_panel_width: &rgis_ui::SidePanelWidth,
    top_panel_height: &rgis_ui::TopPanelHeight,
    bottom_panel_height: &rgis_ui::BottomPanelHeight,
) {
    let layer_center = bounding_rect.0.center();
    let canvas_size = rgis_units::map_area_size(
        window,
        rgis_units::ScreenLength(side_panel_width.0),
        rgis_units::ScreenLength(top_panel_height.0),
        rgis_units::ScreenLength(bottom_panel_height.0),
    );

    let scale = determine_scale(bounding_rect.0, canvas_size.0);
    let camera_scale = crate::CameraScale(scale as f32);
    let mut camera_offset = crate::CameraOffset::from_coord(layer_center);
    camera_offset.pan_x(-side_panel_width.0 / 2., camera_scale);
    camera_offset.pan_y(
        (top_panel_height.0 - bottom_panel_height.0) / 2.,
        camera_scale,
    );
    set_camera_transform(camera_transform, camera_offset, camera_scale);
}

pub(crate) fn set_camera_transform(
    transform: &mut Transform,
    camera_offset: crate::CameraOffset,
    camera_scale: crate::CameraScale,
) {
    transform.translation = camera_offset.to_transform_translation_vec();
    transform.scale = camera_scale.to_transform_scale_vec();
    debug!("New transform scale: {:?}", transform.scale);
}

pub(crate) fn determine_scale(bounding_rect: geo::Rect, canvas_size: bevy::ui::Size<f32>) -> f32 {
    (bounding_rect.width() as f32 / canvas_size.width)
        .max(bounding_rect.height() as f32 / canvas_size.height)
}
