use bevy::prelude::*;

pub(crate) fn center_camera_on_projected_world_rect(
    bounding_rect: geo_projected::Projected<geo::Rect>,
    camera_transform: &mut Transform,
    map_area: rgis_units::MapArea,
) {
    let layer_center = bounding_rect.0.center();
    let scale = determine_scale(bounding_rect.0, map_area.size().to_bevy_size());
    let camera_scale = crate::CameraScale(scale);
    let mut camera_offset = crate::CameraOffset::from_coord(layer_center);
    camera_offset.pan_x(
        (map_area.right_offset_px - map_area.left_offset_px) / 2.,
        camera_scale,
    );
    camera_offset.pan_y(
        (map_area.top_offset_px - map_area.bottom_offset_px) / 2.,
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

pub(crate) fn determine_scale(bounding_rect: geo::Rect, canvas_size: bevy::ui::Size) -> f32 {
    let width = match canvas_size.width {
        Val::Px(p) => p,
        _ => unreachable!(),
    };
    let height = match canvas_size.height {
        Val::Px(p) => p,
        _ => unreachable!(),
    };
    (bounding_rect.width() as f32 / width).max(bounding_rect.height() as f32 / height)
}
