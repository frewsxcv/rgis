use crate::layer::Layers;
use glutin::dpi::PhysicalSize;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest};
use pathfinder_color::ColorF;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{vec2i, Vector2F, Vector2I};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_renderer::options::RenderTransform;
use pathfinder_resources::fs::FilesystemResourceLoader;
use std::sync;

pub const WINDOW_SIZE_X: i32 = 800;
pub const WINDOW_SIZE_Y: i32 = 800;

#[derive(Debug)]
pub enum UserEvent {
    LayerAdded,
}

pub struct Window {
    scene_proxy: SceneProxy,
    pub event_loop: EventLoop<UserEvent>,
    renderer: Renderer<GLDevice>,
    gl_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    layers: sync::Arc<sync::RwLock<Layers>>,
}

impl Window {
    pub fn new(layers: sync::Arc<sync::RwLock<Layers>>) -> Self {
        let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
        let window_size = vec2i(WINDOW_SIZE_X, WINDOW_SIZE_Y);
        let physical_window_size =
            PhysicalSize::new(window_size.x() as f64, window_size.y() as f64);

        // Open a window.
        let window_builder = WindowBuilder::new()
            .with_title(crate::PROGRAM_NAME)
            .with_inner_size(physical_window_size);

        // Create an OpenGL 3.x context for Pathfinder to use.
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .with_gl_profile(GlProfile::Core)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        // Load OpenGL, and make the context current.
        let gl_context = unsafe { gl_context.make_current().unwrap() };
        gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        // Create a Pathfinder renderer.
        let renderer = Renderer::new(
            GLDevice::new(GLVersion::GL3, 0),
            &FilesystemResourceLoader::locate(),
            DestFramebuffer::full_window(window_size),
            RendererOptions {
                background_color: Some(ColorF::white()),
            },
        );

        let scene_proxy = SceneProxy::new(RayonExecutor);

        Window {
            event_loop,
            scene_proxy,
            renderer,
            gl_context,
            layers,
        }
    }

    pub fn start_event_loop(self) -> ! {
        let Window {
            scene_proxy,
            event_loop,
            renderer,
            gl_context,
            layers,
        } = self;

        let window_size = vec2i(WINDOW_SIZE_X, WINDOW_SIZE_Y);

        let mut ctx = EventLoopContext {
            scene_proxy: scene_proxy,
            renderer: renderer,
            gl_context: gl_context,
            layers: layers,
            window_size: window_size,
            view_box: RectF::new(
                Vector2F::new(0., 0.),
                Vector2F::new(WINDOW_SIZE_X as f32, WINDOW_SIZE_Y as f32),
            ),
            // The initial bounding rectangle value doesn't matter. It'll get
            // populated with a meaningful value after we load the first layer.
            bounding_rect: RectF::new(
                Vector2F::new(0., 0.),
                Vector2F::new(WINDOW_SIZE_X as f32, WINDOW_SIZE_Y as f32),
            ),
            // The initial scale value doesn't matter. It'll get populated with
            // a meaningful value after we load the first layer.
            scale: 1.,
            resized: false,
        };

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(_) => handle_redraw_requested(&mut ctx),
                Event::UserEvent(user_event) => handle_user_event(&mut ctx, user_event),
                Event::WindowEvent {
                    event: window_event,
                    ..
                } => handle_window_event(&mut ctx, window_event, control_flow),
                _ => *control_flow = ControlFlow::Wait,
            };
        })
    }
}

struct EventLoopContext {
    scene_proxy: SceneProxy,
    renderer: Renderer<GLDevice>,
    gl_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    window_size: Vector2I,
    layers: sync::Arc<sync::RwLock<Layers>>,
    view_box: pathfinder_geometry::rect::RectF,
    bounding_rect: pathfinder_geometry::rect::RectF,
    scale: f32,
    resized: bool,
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
        ctx.scale = (ctx.window_size.x() as f32 / ctx.bounding_rect.width())
            .min(ctx.window_size.y() as f32 / ctx.bounding_rect.height());
        let layers: &Layers = &ctx.layers.read().unwrap();
        let canvas = crate::render(ctx.window_size, layers, ctx.scale);
        ctx.scene_proxy
            .replace_scene(canvas.into_canvas().into_scene());
        ctx.resized = false;
    }

    ctx.scene_proxy.set_view_box(ctx.view_box);

    let transform = Transform2F::from_scale(Vector2F::splat(ctx.scale as f32));

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

fn handle_window_event(
    ctx: &mut EventLoopContext,
    window_event: WindowEvent,
    control_flow: &mut ControlFlow,
) {
    match window_event {
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        WindowEvent::Resized(window_size) => {
            ctx.window_size = vec2i(window_size.width as i32, window_size.height as i32);
            ctx.resized = true;
            ctx.gl_context.window().request_redraw();
        }
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Up),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            ctx.view_box = ctx.view_box + Vector2F::new(0., 10.);
            ctx.gl_context.window().request_redraw();
        }
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Down),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            ctx.view_box = ctx.view_box + Vector2F::new(0., -10.);
            ctx.gl_context.window().request_redraw();
        }
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            ctx.view_box = ctx.view_box + Vector2F::new(10., 0.);
            ctx.gl_context.window().request_redraw();
        }
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            ctx.view_box = ctx.view_box + Vector2F::new(-10., 0.);
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
