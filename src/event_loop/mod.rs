use rgis_layers::Layers;
use crate::window::UserEvent;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::event::Event;
use glutin::event::{ModifiersState, WindowEvent};
use glutin::event_loop::ControlFlow;

use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{vec2i, Vector2F};

use pathfinder_renderer::gpu::options::DestFramebuffer;

use pathfinder_renderer::options::BuildOptions;
use pathfinder_renderer::options::RenderTransform;

mod keyboard_input;
mod mouse_input;

mod context;
pub use context::EventLoopContext;

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
        *ctx.renderer.options_mut() = pathfinder_renderer::gpu::options::RendererOptions {
            dest: DestFramebuffer::full_window(ctx.window_size),
            background_color: Some(crate::bg_color()),
            show_debug_ui: crate::SHOW_DEBUG_UI,
        };
        ctx.gl_context.resize(PhysicalSize::new(
            ctx.window_size.x() as u32,
            ctx.window_size.y() as u32,
        ));
        ctx.resized = false;
    }

    ctx.scene_proxy.set_view_box(ctx.view_box);

    let transform = Transform2F::from_scale(Vector2F::splat(ctx.scale as f32))
        * Transform2F::from_translation(-ctx.view_center);

    let options = BuildOptions {
        transform: RenderTransform::Transform2D(transform),
        dilation: Vector2F::default(),
        subpixel_aa_enabled: false,
    };

    let t = ::std::time::Instant::now();

    ctx.scene_proxy.build_and_render(&mut ctx.renderer, options);
    ctx.gl_context.swap_buffers().unwrap();

    log::debug!("Rerender time: {} ms", t.elapsed().as_millis());
}

fn handle_user_event(ctx: &mut EventLoopContext, user_event: UserEvent) {
    match user_event {
        UserEvent::LayerAdded => handle_user_event_layer_added(ctx),
    }
}

fn handle_user_event_layer_added(ctx: &mut EventLoopContext) {
    let canvas = {
        let layers: &Layers = &ctx.layers.read().unwrap();
        let geo_bounding_rect = layers.projected_bounding_rect.unwrap();
        ctx.canvas_bounding_rect = geo_bounding_rect_to_canvas_bounding_rect(geo_bounding_rect);
        ctx.scale = (ctx.window_size.x() as f32 / ctx.canvas_bounding_rect.width())
            .min(ctx.window_size.y() as f32 / ctx.canvas_bounding_rect.height());
        ctx.view_center = ctx.canvas_bounding_rect.origin();
        crate::canvas::build(ctx.window_size, layers, ctx.scale)
    };
    ctx.replace_scene_canvas(canvas);
    ctx.gl_context.window().request_redraw();
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
        } => keyboard_input::handle(ctx, keyboard_input, control_flow),
        WindowEvent::CursorMoved { position, .. } => handle_cursor_moved(ctx, position),
        WindowEvent::MouseInput { state, button, .. } => mouse_input::handle(ctx, state, button),
        _ => {
            *control_flow = ControlFlow::Wait;
        }
    }
}

fn handle_modifiers_changed(ctx: &mut EventLoopContext, modifiers: ModifiersState) {
    ctx.shift_pressed = modifiers.shift();
}

fn handle_cursor_moved(ctx: &mut EventLoopContext, position: PhysicalPosition<f64>) {
    ctx.cursor_position = position;
}

fn geo_bounding_rect_to_canvas_bounding_rect(geo_rect: geo_srs::RectWithSrs<f64>) -> RectF {
    // Invert the y-origin because we're translating to screen coordinates
    RectF::new(
        Vector2F::new(geo_rect.rect.min().x as f32, -geo_rect.rect.max().y as f32),
        Vector2F::new(geo_rect.rect.width() as f32, geo_rect.rect.height() as f32),
    )
}
