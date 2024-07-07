//! An example testing the continuity and correct mapping of the `CubeSphereGrid` projection.

use std::f64::consts::PI;

use pixels::{SurfaceTexture, Pixels};
use surface_grid::{sphere::{CubeSphereGrid, CubeSpherePoint, SpherePoint}, SurfaceGrid, GridPoint};
use winit::{application::ApplicationHandler, dpi::{LogicalSize, PhysicalSize}, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowAttributes, WindowId}};

// The initial window size.
const WINDOW_WIDTH: usize = 720;
const WINDOW_HEIGHT: usize = 480;

struct TestApp {
    window: Option<Window>,
    size: Option<PhysicalSize<u32>>,
    pixels: Option<Pixels>,
    buffer: CubeSphereGrid<(f64, f64, f64), 64>,
}

impl TestApp {
    pub fn new() -> Self {
        Self {
            window: None,
            size: None,
            pixels: None,
            buffer: CubeSphereGrid::from_fn(|point| point.position(1.0))
        }
    }
}

impl ApplicationHandler for TestApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        // Build the window.
        let window_attrs = WindowAttributes::default()
            .with_title("Cube Test")
            .with_inner_size(size);

        self.window = Some(event_loop.create_window(window_attrs).expect("Failed to create window."));

        // Pixels setup.
        let window_size = self.window.as_ref().unwrap().inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, self.window.as_ref().unwrap());

        self.size = Some(window_size);

        self.pixels = Some(Pixels::new(window_size.width, window_size.height, surface_texture)
            .expect("Failed to create pixel buffer."));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(window_size) => {
                // Handle resizing.
                if window_size.width != 0 && window_size.height != 0 {
                    self.size = Some(PhysicalSize::new(window_size.width, window_size.height));

                    self.pixels.as_mut().unwrap().resize_buffer(self.size.as_ref().unwrap().width, self.size.as_ref().unwrap().height)
                        .expect("Failed to resize buffer");
                    self.pixels.as_mut().unwrap().resize_surface(window_size.width, window_size.height)
                        .expect("Failed to resize surface");
                }

                self.window.as_ref().unwrap().request_redraw()
            },
            WindowEvent::CloseRequested => {
                event_loop.exit()
            },
            WindowEvent::RedrawRequested => {
                // Display the result using pixels.
                let frame = self.pixels.as_mut().unwrap().frame_mut();
        
                for y in 0..self.size.as_ref().unwrap().height {
                    for x in 0..self.size.as_ref().unwrap().width {
                        let i = (y as usize * self.size.as_ref().unwrap().width as usize + x as usize) * 4;

                        // Convert the X Y screen coordinates to an equirectangular
                        // projection of the latitude and longitude.
                        let latitude = (y as f64 / self.size.as_ref().unwrap().height as f64) * PI - PI / 2.0;
                        let longitude = (x as f64 / self.size.as_ref().unwrap().width as f64) * PI * 2.0;

                        let (x, y, z) = self.buffer[CubeSpherePoint::from_geographic(latitude, longitude)];

                        frame[i + 0] = ((x + 1.0) / 2.0 * 255.0) as u8;
                        frame[i + 1] = ((y + 1.0) / 2.0 * 255.0) as u8;
                        frame[i + 2] = ((z + 1.0) / 2.0 * 255.0) as u8;
                        frame[i + 3] = 255;
                    }
                }

                // Render the pixels to the screen.
                self.pixels.as_ref().unwrap().render().expect("Failed to render");
            },
            _ => {}
        }
    }
}

fn main() {
    // This example uses winit with pixels to display the game.
    let event_loop = EventLoop::new()
        .expect("Failed to start event loop.");

    let mut app = TestApp::new();

    event_loop.run_app(&mut app)
        .expect("Failed to run app.");
}

