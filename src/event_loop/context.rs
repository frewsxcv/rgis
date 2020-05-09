use crate::layer::Layers;

use glutin::dpi::PhysicalPosition;

use pathfinder_geometry::rect::RectF;

use pathfinder_geometry::vector::{Vector2F, Vector2I};
use pathfinder_gl::GLDevice;

use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;

use pathfinder_renderer::gpu::renderer::Renderer;

use std::sync;

const ZOOM_FACTOR: f32 = 1.1;

pub struct EventLoopContext {
    pub scene_proxy: SceneProxy,
    pub renderer: Renderer<GLDevice>,
    pub gl_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    pub window_size: Vector2I,
    pub layers: sync::Arc<sync::RwLock<Layers>>,
    pub view_box: RectF,
    pub view_center: Vector2F,
    pub canvas_bounding_rect: RectF,
    pub scale: f32,
    pub resized: bool,
    pub shift_pressed: bool,
    pub cursor_position: PhysicalPosition<f64>,
}

impl EventLoopContext {
    pub fn new(
        scene_proxy: SceneProxy,
        renderer: Renderer<GLDevice>,
        gl_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
        layers: sync::Arc<sync::RwLock<Layers>>,
        window_size: Vector2I,
    ) -> Self {
        EventLoopContext {
            scene_proxy: scene_proxy,
            renderer: renderer,
            gl_context: gl_context,
            layers: layers,
            window_size: window_size,
            view_box: RectF::new(
                Vector2F::new(0., 0.),
                Vector2F::new(window_size.x() as f32, window_size.y() as f32),
            ),
            // The initial bounding rectangle value doesn't matter. It'll get
            // populated with a meaningful value after we load the first layer.
            canvas_bounding_rect: RectF::new(Vector2F::new(0., 0.), Vector2F::new(1., 1.)),
            // The initial view center value doesn't matter. It'll get populated
            // with a meaningful value after we load the first layer.
            view_center: Vector2F::new(1., 1.),
            // The initial scale value doesn't matter. It'll get populated with
            // a meaningful value after we load the first layer.
            scale: 1.,
            // The initial cursor position value doesn't matter. It'll get populated with
            // a meaningful value after the mouse is moved.
            cursor_position: PhysicalPosition::new(1., 1.),
            shift_pressed: false,
            resized: false,
        }
    }

    pub fn build_canvas(&mut self) {
        let canvas =
            crate::build_canvas(self.window_size, &self.layers.read().unwrap(), self.scale);
        self.scene_proxy
            .replace_scene(canvas.into_canvas().into_scene());
    }

    pub fn zoom_in(&mut self) {
        self.scale *= ZOOM_FACTOR;
        self.build_canvas();
        self.gl_context.window().request_redraw();
    }

    pub fn zoom_out(&mut self) {
        self.scale /= ZOOM_FACTOR;
        self.build_canvas();
        self.gl_context.window().request_redraw();
    }
}
