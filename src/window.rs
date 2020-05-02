use glutin::dpi::PhysicalSize;
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::fs::FilesystemResourceLoader;

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
}

impl Window {
    pub fn new() -> Self {
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
        }
    }

    pub fn start_event_loop(self) -> ! {
        let Window {
            scene_proxy,
            event_loop,
            mut renderer,
            gl_context,
        } = self;

        // TODO: this is wrong
        let window_size = vec2i(WINDOW_SIZE_X, WINDOW_SIZE_Y);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::UserEvent(UserEvent::Render) => {
                    let canvas = crate::render(window_size);
                    scene_proxy.replace_scene(canvas.into_canvas().into_scene());
                    scene_proxy.build_and_render(&mut renderer, BuildOptions::default());
                    gl_context.swap_buffers().unwrap();
                }
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
                    let window_size_i = vec2i(window_size.width as i32, window_size.height as i32);
                    renderer.replace_dest_framebuffer(DestFramebuffer::full_window(window_size_i));
                    gl_context.resize(PhysicalSize::new(window_size.width as u32, window_size.height as u32));
                    let canvas = crate::render(window_size_i);
                    scene_proxy.replace_scene(canvas.into_canvas().into_scene());
                    scene_proxy.build_and_render(&mut renderer, BuildOptions::default());
                    gl_context.swap_buffers().unwrap();
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::Up),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    let view_box = scene_proxy.copy_scene().view_box();
                    scene_proxy.set_view_box(pathfinder_geometry::rect::RectF::new(
                        pathfinder_geometry::vector::Vector2F::new(
                            view_box.min_x(),
                            view_box.min_y() + 10.,
                        ),
                        pathfinder_geometry::vector::Vector2F::new(
                            view_box.max_x(),
                            view_box.max_y() + 10.,
                        ),
                    ));
                    scene_proxy.build_and_render(&mut renderer, BuildOptions::default());
                    gl_context.swap_buffers().unwrap();
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::Down),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    let view_box = scene_proxy.copy_scene().view_box();
                    scene_proxy.set_view_box(pathfinder_geometry::rect::RectF::new(
                        pathfinder_geometry::vector::Vector2F::new(
                            view_box.min_x(),
                            view_box.min_y() - 10.,
                        ),
                        pathfinder_geometry::vector::Vector2F::new(
                            view_box.max_x(),
                            view_box.max_y() - 10.,
                        ),
                    ));
                    scene_proxy.build_and_render(&mut renderer, BuildOptions::default());
                    gl_context.swap_buffers().unwrap();
                }
                _ => {
                    *control_flow = ControlFlow::Wait;
                }
            };
        })
    }

    // pub fn resize(&mut self, size: Vector2F) {
    //     // let new_framebuffer_size = size.to_i32();
    //     // if new_framebuffer_size != self.framebuffer_size {
    //     //     self.framebuffer_size = new_framebuffer_size;
    //     //     self.windowed_context.resize(PhysicalSize::new(self.framebuffer_size.x() as u32, self.framebuffer_size.y() as u32));
    //     //     self.renderer.replace_dest_framebuffer(DestFramebuffer::full_window(self.framebuffer_size));
    //     // }
    // }
}
