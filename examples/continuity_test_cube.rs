//! An example testing the continuity and correct mapping of the `CubeSphereGrid` projection.

use std::{error::Error, f64::consts::PI};

use pixels::{SurfaceTexture, Pixels};
use surface_grid::{sphere::{CubeSphereGrid, CubeSpherePoint, SpherePoint}, SurfaceGrid};
use winit::{event_loop::EventLoop, window::WindowBuilder, dpi::{LogicalSize, PhysicalSize}, event::{Event, WindowEvent}};

// The initial window size.
const WINDOW_WIDTH: usize = 720;
const WINDOW_HEIGHT: usize = 480;

fn main() -> Result<(), Box<dyn Error>> {
    // This example uses winit with pixels to display the game.
    let event_loop = EventLoop::new()?;

    let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);

    // Build the window.
    let window = WindowBuilder::new()
        .with_title("Continuity Test")
        .with_inner_size(size)
        .build(&event_loop)?;

    // Pixels setup.
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let mut size = window_size;

    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

    let buffer: CubeSphereGrid<(f64, f64), 20> = CubeSphereGrid::from_fn(|point| (point.latitude(), point.longitude()));

    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(window_size) => {
                        // Handle resizing.
                        if window_size.width != 0 && window_size.height != 0 {
                            size = PhysicalSize::new(window_size.width, window_size.height);

                            pixels.resize_buffer(size.width, size.height)
                                .expect("Failed to resize buffer");
                            pixels.resize_surface(window_size.width, window_size.height)
                                .expect("Failed to resize surface");
                        }

                        window.request_redraw()
                    },
                    WindowEvent::CloseRequested => {
                        target.exit()
                    }
                    WindowEvent::RedrawRequested => {
                        // Display the result using pixels.
                        let frame = pixels.frame_mut();
                
                        for y in 0..size.height {
                            for x in 0..size.width {
                                let i = (y as usize * size.width as usize + x as usize) * 4;

                                // Convert the X Y screen coordinates to an equirectangular
                                // projection of the latitude and longitude.
                                let latitude = (y as f64 / size.height as f64) * PI - PI / 2.0;
                                let longitude = (x as f64 / size.width as f64) * PI * 2.0;

                                // Gets the value stored at the latitude and longitude calculated.
                                let (lat, long) = buffer[CubeSpherePoint::from_geographic(latitude, longitude)];

                                frame[i] = ((lat + PI / 2.0) / PI * 255.0) as u8;
                                frame[i + 2] = (long / (2.0 * PI) * 255.0) as u8;
                                frame[i + 3] = 255;
                            }
                        }

                        // Render the pixels to the screen.
                        pixels.render().expect("Failed to render");
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    })?;

    Ok(())
}

