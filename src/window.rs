use crate::layer::Layers;
use glutin::dpi::PhysicalSize;
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::{vec2i, Vector2I};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::fs::FilesystemResourceLoader;
use std::sync;

pub const WINDOW_SIZE_X: i32 = 800;
pub const WINDOW_SIZE_Y: i32 = 800;

#[derive(Debug)]
pub enum UserEvent {
    Render,
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

        // Render the canvas to screen.

        // let event_loop_proxy = event_loop.create_proxy();

        // std::thread::spawn(move || {
        //     event_loop_proxy.send_event(UserEvent::Render);
        // });

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
            // TODO: this never gets updated
            window_size: window_size,
        };

        event_loop.run(move |event, _, control_flow| {
            match event {
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
}

fn handle_user_event(ctx: &mut EventLoopContext, user_event: UserEvent) {
    match user_event {
        UserEvent::Render => {
            let canvas = crate::render(ctx.window_size, ctx.layers.clone());
            ctx.scene_proxy
                .replace_scene(canvas.into_canvas().into_scene());
            ctx.scene_proxy
                .build_and_render(&mut ctx.renderer, BuildOptions::default());
            ctx.gl_context.swap_buffers().unwrap();
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
            let window_size_i = vec2i(window_size.width as i32, window_size.height as i32);
            ctx.renderer
                .replace_dest_framebuffer(DestFramebuffer::full_window(window_size_i));
            ctx.gl_context.resize(PhysicalSize::new(
                window_size.width as u32,
                window_size.height as u32,
            ));
            let canvas = crate::render(window_size_i, ctx.layers.clone());
            ctx.scene_proxy
                .replace_scene(canvas.into_canvas().into_scene());
            ctx.scene_proxy
                .build_and_render(&mut ctx.renderer, BuildOptions::default());
            ctx.gl_context.swap_buffers().unwrap();
        }
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Up),
                    ..
                },
            ..
        } => {
            let view_box = ctx.scene_proxy.copy_scene().view_box();
            ctx.scene_proxy
                .set_view_box(pathfinder_geometry::rect::RectF::new(
                    pathfinder_geometry::vector::Vector2F::new(
                        view_box.min_x(),
                        view_box.min_y() + 10.,
                    ),
                    pathfinder_geometry::vector::Vector2F::new(
                        view_box.max_x(),
                        view_box.max_y() + 10.,
                    ),
                ));
            ctx.scene_proxy
                .build_and_render(&mut ctx.renderer, BuildOptions::default());
            ctx.gl_context.swap_buffers().unwrap();
        }
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Down),
                    ..
                },
            ..
        } => {
            let view_box = ctx.scene_proxy.copy_scene().view_box();
            ctx.scene_proxy
                .set_view_box(pathfinder_geometry::rect::RectF::new(
                    pathfinder_geometry::vector::Vector2F::new(
                        view_box.min_x(),
                        view_box.min_y() - 10.,
                    ),
                    pathfinder_geometry::vector::Vector2F::new(
                        view_box.max_x(),
                        view_box.max_y() - 10.,
                    ),
                ));
            ctx.scene_proxy
                .build_and_render(&mut ctx.renderer, BuildOptions::default());
            ctx.gl_context.swap_buffers().unwrap();
        }
        _ => {
            *control_flow = ControlFlow::Wait;
        }
    }
}
