use glutin::dpi::PhysicalSize;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use pathfinder_geometry::vector::{vec2i};
use glutin::{ContextBuilder, GlProfile, GlRequest};
use pathfinder_canvas::{Canvas, CanvasFontContext, CanvasRenderingContext2D};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::fs::FilesystemResourceLoader;
use pathfinder_gl::{GLDevice, GLVersion};
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use pathfinder_color::ColorF;

pub const WINDOW_SIZE_X: i32 = 800;
pub const WINDOW_SIZE_Y: i32 = 800;

pub fn build_window<F>(f: F)
where
    F: Fn(&mut CanvasRenderingContext2D),
{
    // Calculate the right logical size of the window.
    let event_loop = EventLoop::new();
    let window_size = vec2i(WINDOW_SIZE_X, WINDOW_SIZE_Y);
    let physical_window_size = PhysicalSize::new(window_size.x() as f64, window_size.y() as f64);

    // Open a window.
    let window_builder = WindowBuilder::new().with_title("Minimal example")
                                                .with_inner_size(physical_window_size);

    // Create an OpenGL 3.x context for Pathfinder to use.
    let gl_context = ContextBuilder::new().with_gl(GlRequest::Latest)
                                            .with_gl_profile(GlProfile::Core)
                                            .build_windowed(window_builder, &event_loop)
                                            .unwrap();

    // Load OpenGL, and make the context current.
    let gl_context = unsafe { gl_context.make_current().unwrap() };
    gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

    // Create a Pathfinder renderer.
    let mut renderer = Renderer::new(GLDevice::new(GLVersion::GL3, 0),
                                        &FilesystemResourceLoader::locate(),
                                        DestFramebuffer::full_window(window_size),
                                        RendererOptions { background_color: Some(ColorF::white()) });

    // Make a canvas. We're going to draw a house.
    let font_context = CanvasFontContext::from_system_source();
    let mut canvas = Canvas::new(window_size.to_f32()).get_context_2d(font_context);

    f(&mut canvas);

    // Render the canvas to screen.
    let scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(), RayonExecutor);
    scene.build_and_render(&mut renderer, BuildOptions::default());
    gl_context.swap_buffers().unwrap();

    // Wait for a keypress.
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } |
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. },
                    ..
                },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => {
                *control_flow = ControlFlow::Wait;
            },
        };
    })
}
