#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_functions

@group(1) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(1) var<uniform> width: f32;

@vertex
fn vertex(
    @location(0) vert_pos: vec3<f32>,
    @location(1) point_a: vec3<f32>,
    @location(2) point_b: vec3<f32>,
) -> @builtin(position) vec4<f32> {
    let x_basis = point_b.xy - point_a.xy;
    let y_basis = normalize(vec2<f32>(-x_basis.y, x_basis.x));

    let point = point_a.xy + x_basis * vert_pos.x + y_basis * width * vert_pos.y;

    let world_position = vec4<f32>(point.xy, 0.0, 1.0);

    return mesh_view_bindings::view.view_proj * mesh_view_bindings::globals.model * world_position;
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return color;
}
