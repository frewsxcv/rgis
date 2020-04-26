use glutin::dpi::PhysicalSize;
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::vec2i;
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::fs::FilesystemResourceLoader;

pub const WINDOW_SIZE_X: i32 = 800;
pub const WINDOW_SIZE_Y: i32 = 800;

enum UserEvent {
    Render,
}

pub fn build_window() {
    // Calculate the right logical size of the window.
    let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
    let window_size = vec2i(WINDOW_SIZE_X, WINDOW_SIZE_Y);
    let physical_window_size = PhysicalSize::new(window_size.x() as f64, window_size.y() as f64);

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
    let mut renderer = Renderer::new(
        GLDevice::new(GLVersion::GL3, 0),
        &FilesystemResourceLoader::locate(),
        DestFramebuffer::full_window(window_size),
        RendererOptions {
            background_color: Some(ColorF::white()),
        },
    );

    /////////
    ::std::thread::sleep(::std::time::Duration::from_secs(3));
    let canvas = crate::render(window_size);
    /////////

    // Render the canvas to screen.
    let scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(), RayonExecutor);
    scene.build_and_render(&mut renderer, BuildOptions::default());
    gl_context.swap_buffers().unwrap();

    // let event_loop_proxy = event_loop.create_proxy();

    // std::thread::spawn(move || {
    //     event_loop_proxy.send_event(UserEvent::Render);
    // });

    // Wait for a keypress.
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(window_size),
                ..
            } => {
                let window_size = vec2i(window_size.width as i32, window_size.height as i32);
                renderer.set_main_framebuffer_size(window_size);
                let canvas = crate::render(window_size);
                scene.replace_scene(canvas.into_canvas().into_scene());
                // scene.set_view_box(pathfinder_geometry::rect::RectF::new(
                //     pathfinder_geometry::vector::Vector2F::zero(),
                //     window_size.to_f32(),
                // ));
                scene.build_and_render(&mut renderer, BuildOptions::default());
                gl_context.swap_buffers().unwrap();
            }
            _ => {
                *control_flow = ControlFlow::Wait;
            }
        };
    })
}
