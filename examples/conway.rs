//! An example implementing conways game of life on the surface of a sphere.

use std::{f64::consts::PI, mem::swap, time::{Duration, Instant}};

use pixels::{SurfaceTexture, Pixels};
use rand::{thread_rng, Rng};
use surface_grid::{sphere::{CubeSphereGrid, CubeSpherePoint, SpherePoint}, SurfaceGrid};
use winit::{application::ApplicationHandler, dpi::{LogicalSize, PhysicalSize}, event::{StartCause, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowAttributes, WindowId}};

// The initial window size.
const WINDOW_WIDTH: usize = 720;
const WINDOW_HEIGHT: usize = 480;

struct ConwayGameOfLife {
    window: Option<Window>,
    pixels: Option<Pixels>,
    buffer1: CubeSphereGrid<bool, 256>,
    buffer2: CubeSphereGrid<bool, 256>,
    size: Option<PhysicalSize<u32>>,
}

impl ConwayGameOfLife {
    pub fn new() -> Self {
        // Create two grids to swap between.
        // This saves allocating a new grid for each frame.
        let mut rng = thread_rng();

        // The size specified here might be smaller than expected.
        // This is because it is the size of each cube face rather than the size of the whole grid.
        // A size of 512 leads to 1572864 grid cells. This is equivalent to an image around 1500x1500.
        let buffer1: CubeSphereGrid<bool, 256> = CubeSphereGrid::from_fn(|_| rng.gen());
        let buffer2: CubeSphereGrid<bool, 256> = CubeSphereGrid::default();
        
        Self {
            window: None,
            pixels: None,
            size: None,
            buffer1,
            buffer2,
        }
    }
}

impl ApplicationHandler for ConwayGameOfLife {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        // Build the window.
        let window_attrs = WindowAttributes::default()
            .with_title("Conway's Game of Life")
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
                // Calculate conways game of life in parallel.
                self.buffer2.set_from_neighbours_diagonals_par(&self.buffer1, |s1, s2, s3, s4, current, s6, s7, s8, s9| {
                    let count = [s1, s2, s3, s4, s6, s7, s8, s9]
                        .into_iter()
                        .filter(|s| **s)
                        .count();

                    if count < 2 {
                        false
                    } else if count > 3 {
                        false
                    } else if *current && count == 2 {
                        true
                    } else if count == 3 {
                        true
                    } else {
                        false
                    }
                });

                // Swap the buffers.
                swap(&mut self.buffer2, &mut self.buffer1);

                // Display the result using pixels.
                let frame = self.pixels.as_mut().unwrap().frame_mut();
        
                for y in 0..self.size.as_ref().unwrap().height {
                    for x in 0..self.size.as_ref().unwrap().width {
                        let i = (y as usize * self.size.as_ref().unwrap().width as usize + x as usize) * 4;

                        // Convert the X Y screen coordinates to an equirectangular
                        // projection of the latitude and longitude.
                        let latitude = (y as f64 / self.size.as_ref().unwrap().height as f64) * PI - PI / 2.0;
                        let longitude = (x as f64 / self.size.as_ref().unwrap().width as f64) * PI * 2.0;

                        // Gets the value stored at the latitude and longitude calculated.
                        let value = self.buffer1[CubeSpherePoint::from_geographic(latitude, longitude)];

                        // Set the pixel colour.
                        if value {
                            frame[i] = 255;
                            frame[i + 1] = 255;
                            frame[i + 2] = 255;
                        } else {
                            frame[i] = 0;
                            frame[i + 1] = 0;
                            frame[i + 2] = 0;
                        }
                        frame[i + 3] = 255;
                    }
                }

                // Render the pixels to the screen.
                self.pixels.as_ref().unwrap().render().expect("Failed to render");
            },
            _ => {}
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::ResumeTimeReached { .. } => {
                self.window.as_ref().unwrap().request_redraw();
            },
            StartCause::Init => {
                event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(1000 / 60)))
            },
            _ => {}
        }
    }
}

fn main() {
    // This example uses winit with pixels to display the game.
    let event_loop = EventLoop::new()
        .expect("Failed to start event loop.");

    let mut app = ConwayGameOfLife::new();

    event_loop.run_app(&mut app)
        .expect("Failed to run app.");
}
