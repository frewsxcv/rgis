use crate::layer::Layers;
use crate::window::UserEvent;
use glutin::dpi::PhysicalSize;
use glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent};
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
    pub view_box: pathfinder_geometry::rect::RectF,
    pub view_center: Vector2F,
    pub bounding_rect: pathfinder_geometry::rect::RectF,
    pub scale: f32,
    pub resized: bool,
    pub shift_pressed: bool,
}

pub fn handle_redraw_requested(ctx: &mut EventLoopContext) {
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

pub fn handle_user_event(ctx: &mut EventLoopContext, user_event: UserEvent) {
    match user_event {
        UserEvent::LayerAdded => {
            let layers: &Layers = &ctx.layers.read().unwrap();
            let geo_bounding_rect = layers.bounding_rect.unwrap();
            ctx.bounding_rect = geo_rect_to_pathfinder_rect(geo_bounding_rect);
            ctx.scale = (ctx.window_size.x() as f32 / ctx.bounding_rect.width())
                .min(ctx.window_size.y() as f32 / ctx.bounding_rect.height());
            let canvas = crate::render(ctx.window_size, layers, ctx.scale);
            ctx.scene_proxy
                .replace_scene(canvas.into_canvas().into_scene());
            ctx.gl_context.window().request_redraw();
        }
    }
}

pub fn handle_window_event(
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

fn geo_rect_to_pathfinder_rect(geo_rect: geo::Rect<f64>) -> RectF {
    RectF::new(
        Vector2F::new(geo_rect.min().x as f32, geo_rect.max().y as f32),
        Vector2F::new(geo_rect.width() as f32, geo_rect.height() as f32),
    )
}
