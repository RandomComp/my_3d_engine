mod engine;

use std::iter::zip;
use std::slice::from_raw_parts_mut;
use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::engine::{Color, Engine, Pos2D, Pos2DTyped, Pos3D, Renderer, Surface};

struct Game {
    engine: Engine,
    delta: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            engine: Engine::new(),
            delta: 0.0
        }
    }

    fn update_frame(&mut self, frame: &mut Surface, mouse: (f64, f64)) {
        Renderer::clear(frame, Color::gray(0.2));

        let matrix = self.engine.take_matrix();

        self.engine.translate(Pos2D::new(mouse.0 as f32, mouse.1 as f32));

        self.engine.triangle(
            Color::new(1.0_f32, 0.0_f32, 0.0_f32),

            Pos2D::new(0.0_f32, -60.0_f32),
            Pos2D::new(-60.0_f32, 60.0_f32),
            Pos2D::new(60.0_f32, 60.0_f32),
        );
        self.engine.wf_triangle(
            Color::new(0.0_f32, 1.0_f32, 0.0_f32),

            Pos2D::new(0.0_f32, -60.0_f32),
            Pos2D::new(-60.0_f32, 60.0_f32),
            Pos2D::new(60.0_f32, 60.0_f32),
        );

        Renderer::render(frame, self.engine.take_objects());

        self.engine.load_matrix(matrix);

        self.delta += 0.1;
    }
}

#[derive(Default)]
struct App {
    // Оборачиваем в Arc, чтобы безопасно передать ссылку в Pixels
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    game: Option<Game>,
    mouse: (f64, f64),
}

pub fn cast_u8_to_u32(arr: &mut [u8]) -> &mut [u32] {
    unsafe {
        let ptr = arr.as_mut_ptr();

        from_raw_parts_mut(ptr as *mut u32, arr.len() / 4)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.window.is_none() {
            return
        }

        let mut window_attributes = Window::default_attributes()
            .with_title("My3DEngine");

        #[cfg(target_os = "linux")] {
            use winit::platform::wayland::WindowAttributesExtWayland;
            window_attributes = window_attributes.with_name("3DEngine", "");
        }

        let Ok(window) = event_loop.create_window(window_attributes) else {
            return
        };

        let window = Arc::new(window);
        let size = window.inner_size();

        // Передаем клон Arc<Window> внутрь SurfaceTexture
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());

        if let Ok(pixels) = Pixels::new(size.width, size.height, surface_texture) {
            self.window = Some(window);

            // Запрашиваем первый кадр
            self.window.as_ref().unwrap().request_redraw();

            self.game = Some(Game::new());

            self.pixels = Some(pixels);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let (Some(pixels), Some(game), Some(window)) = (&mut self.pixels, &mut self.game, &self.window) else {
            return
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                pixels.resize_buffer(new_size.width, new_size.height).expect("width or height < 0 or exceeds of GPU memory");
                pixels.resize_surface(new_size.width, new_size.height).expect("width or height < 0 or exceeds of GPU memory");
            }
            WindowEvent::RedrawRequested => {
                let size = window.inner_size();

                let frame = cast_u8_to_u32(pixels.frame_mut());

                let mut surface = Surface { frame, width: size.width, height: size.height };

                game.update_frame(&mut surface, self.mouse);

                if pixels.render().is_err() {
                    event_loop.exit();
                    return;
                }

                window.request_redraw();
            }
            WindowEvent::CursorMoved { device_id: _, position } => {
                self.mouse = position.into();
            }
            _ => {},
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
