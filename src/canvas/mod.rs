use crate::layer::Layers;
use add_to_canvas::AddToCanvas;
use pathfinder_canvas::{Canvas, CanvasFontContext};

mod add_to_canvas;
mod to_canvas_path;

pub fn build(
    window_size: pathfinder_geometry::vector::Vector2I,
    layers: &Layers,
    scale: f32,
) -> pathfinder_canvas::CanvasRenderingContext2D {
    log::info!("Building canvas");
    let mut canvas = new_canvas(window_size);

    // Flip the y-axis for the cartesian → screen coordinate translation.
    canvas.scale(pathfinder_geometry::vector::vec2f(1., -1.));

    for layer in &layers.data {
        layer.projected_geometry.geometry.add_to_canvas(add_to_canvas::Context {
            canvas: &mut canvas,
            scale,
            color: layer.color,
            selected: layers.selected_layer_id == Some(layer.id),
        })
    }

    canvas
}

fn new_canvas(
    window_size: pathfinder_geometry::vector::Vector2I,
) -> pathfinder_canvas::CanvasRenderingContext2D {
    let font_context = CanvasFontContext::from_system_source();
    Canvas::new(window_size.to_f32()).get_context_2d(font_context)
}
