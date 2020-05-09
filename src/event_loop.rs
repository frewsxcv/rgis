use crate::layer::Layers;
use crate::window::UserEvent;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::event::Event;
use glutin::event::{
    ElementState, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode, WindowEvent,
};
use glutin::event_loop::ControlFlow;

use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{vec2i, Vector2F, Vector2I};
use pathfinder_gl::GLDevice;

use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::DestFramebuffer;
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_renderer::options::RenderTransform;

use std::sync;

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
}

pub fn handle_event(
    ctx: &mut EventLoopContext,
    event: Event<UserEvent>,
    control_flow: &mut ControlFlow,
) {
    match event {
        Event::RedrawRequested(_) => handle_redraw_requested(ctx),
        Event::UserEvent(user_event) => handle_user_event(ctx, user_event),
        Event::WindowEvent {
            event: window_event,
            ..
        } => handle_window_event(ctx, window_event, control_flow),
        _ => *control_flow = ControlFlow::Wait,
    }
}

fn handle_redraw_requested(ctx: &mut EventLoopContext) {
    if ctx.resized {
        ctx.view_box = RectF::new(
            Vector2F::new(0., 0.),
            Vector2F::new(ctx.window_size.x() as f32, ctx.window_size.y() as f32),
        );
        ctx.renderer
            .replace_dest_framebuffer(DestFramebuffer::full_window(ctx.window_size));
        ctx.gl_context.resize(PhysicalSize::new(
            ctx.window_size.x() as u32,
            ctx.window_size.y() as u32,
        ));
        ctx.resized = false;
    }

    // ctx.view_center = ctx.view_box.origin() + ctx.view_box.size() * 0.5;

    ctx.scene_proxy.set_view_box(ctx.view_box);

    let transform = Transform2F::from_scale(Vector2F::splat(ctx.scale as f32))
        * Transform2F::from_translation(-ctx.view_center);

    let options = BuildOptions {
        transform: RenderTransform::Transform2D(transform),
        dilation: Vector2F::default(),
        subpixel_aa_enabled: false,
    };

    ctx.scene_proxy.build_and_render(&mut ctx.renderer, options);
    ctx.gl_context.swap_buffers().unwrap();
}

fn handle_user_event(ctx: &mut EventLoopContext, user_event: UserEvent) {
    match user_event {
        UserEvent::LayerAdded => {
            let layers: &Layers = &ctx.layers.read().unwrap();
            let geo_bounding_rect = layers.bounding_rect.unwrap();
            ctx.canvas_bounding_rect = geo_bounding_rect_to_canvas_bounding_rect(geo_bounding_rect);
            ctx.scale = (ctx.window_size.x() as f32 / ctx.canvas_bounding_rect.width())
                .min(ctx.window_size.y() as f32 / ctx.canvas_bounding_rect.height());
            ctx.view_center = ctx.canvas_bounding_rect.origin();
            let canvas = crate::render(ctx.window_size, layers, ctx.scale);
            ctx.scene_proxy
                .replace_scene(canvas.into_canvas().into_scene());
            ctx.gl_context.window().request_redraw();
        }
    }
}

fn handle_window_event(
    ctx: &mut EventLoopContext,
    window_event: WindowEvent,
    control_flow: &mut ControlFlow,
) {
    match window_event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(window_size) => {
            ctx.window_size = vec2i(window_size.width as i32, window_size.height as i32);
            ctx.resized = true;
            ctx.gl_context.window().request_redraw();
        }
        WindowEvent::ModifiersChanged(modifiers) => handle_modifiers_changed(ctx, modifiers),
        WindowEvent::KeyboardInput {
            input: keyboard_input,
            ..
        } => handle_keyboard_input(ctx, keyboard_input, control_flow),
        WindowEvent::CursorMoved { position, .. } => handle_cursor_moved(ctx, position),
        WindowEvent::MouseInput { state, button, .. } => handle_mouse_input(ctx, state, button),
        _ => {
            *control_flow = ControlFlow::Wait;
        }
    }
}

fn handle_modifiers_changed(ctx: &mut EventLoopContext, modifiers: ModifiersState) {
    ctx.shift_pressed = modifiers.shift();
}

const PAN_FACTOR: f32 = 0.05;
const ZOOM_FACTOR: f32 = 1.1;

fn handle_keyboard_input(
    ctx: &mut EventLoopContext,
    keyboard_input: KeyboardInput,
    control_flow: &mut ControlFlow,
) {
    match keyboard_input {
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Escape),
            ..
        } => *control_flow = ControlFlow::Exit,
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Up),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.view_center =
                ctx.view_center - Vector2F::new(0., ctx.view_box.height() * PAN_FACTOR / ctx.scale);
            ctx.gl_context.window().request_redraw();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Down),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.view_center =
                ctx.view_center + Vector2F::new(0., ctx.view_box.height() * PAN_FACTOR / ctx.scale);
            ctx.gl_context.window().request_redraw();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Left),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.view_center =
                ctx.view_center - Vector2F::new(ctx.view_box.width() * PAN_FACTOR / ctx.scale, 0.);
            ctx.gl_context.window().request_redraw();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Right),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.view_center =
                ctx.view_center + Vector2F::new(ctx.view_box.width() * PAN_FACTOR / ctx.scale, 0.);
            ctx.gl_context.window().request_redraw();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Equals),
            state: ElementState::Pressed,
            ..
        } => {
            if ctx.shift_pressed {
                ctx.scale *= ZOOM_FACTOR;
                // ctx.view_center = ctx.view_center + Vector2F::new(10., 0.);
                ctx.gl_context.window().request_redraw();
            }
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Minus),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.scale /= ZOOM_FACTOR;
            ctx.gl_context.window().request_redraw();
        }
        _ => {
            *control_flow = ControlFlow::Wait;
        }
    }
}

fn handle_cursor_moved(ctx: &mut EventLoopContext, position: PhysicalPosition<f64>) {
    ctx.cursor_position = position;
}

fn handle_mouse_input(
    ctx: &mut EventLoopContext,
    element_state: ElementState,
    mouse_button: MouseButton,
) {
    match (mouse_button, element_state) {
        (MouseButton::Left, ElementState::Pressed) => {
            let geo_coordinate = physical_position_to_geo_coordinate(ctx, ctx.cursor_position);
            let mut layers = ctx.layers.write().unwrap();
            let selected_layer_changed = layers.set_selected_layer_from_mouse_press(geo_coordinate);

            log::info!(
                "Mouse clicked. Screen: (x: {}, y: {}). Geo: (x: {}, y: {}).",
                ctx.cursor_position.x,
                ctx.cursor_position.y,
                geo_coordinate.x,
                geo_coordinate.y,
            );

            if selected_layer_changed {
                let canvas = crate::render(ctx.window_size, &layers, ctx.scale);
                ctx.scene_proxy
                    .replace_scene(canvas.into_canvas().into_scene());
                ctx.gl_context.window().request_redraw();
            }
        }
        _ => {}
    }
}

fn physical_position_to_geo_coordinate(
    ctx: &EventLoopContext,
    physical_position: PhysicalPosition<f64>,
) -> geo::Coordinate<f64> {
    geo::Coordinate {
        x: ctx.view_center.x() as f64 + (physical_position.x / (ctx.scale as f64)),
        y: -(ctx.view_center.y() as f64 + (physical_position.y / (ctx.scale as f64))),
    }
}

fn geo_bounding_rect_to_canvas_bounding_rect(geo_rect: geo::Rect<f64>) -> RectF {
    // Invert the y-origin because we're translating to screen coordinates
    RectF::new(
        Vector2F::new(geo_rect.min().x as f32, -geo_rect.max().y as f32),
        Vector2F::new(geo_rect.width() as f32, geo_rect.height() as f32),
    )
}
