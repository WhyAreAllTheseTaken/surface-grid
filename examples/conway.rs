use std::{error::Error, f64::consts::PI, mem::swap, time::{Instant, Duration}};

use pixels::{SurfaceTexture, Pixels};
use rand::{thread_rng, Rng};
use surface_grid::{sphere::{CubeSphereGrid, CubeSpherePoint, SpherePoint}, SurfaceGrid};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::{LogicalSize, PhysicalSize}, event::{Event, WindowEvent, StartCause}};

const WINDOW_WIDTH: usize = 720;
const WINDOW_HEIGHT: usize = 480;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);

    let window = WindowBuilder::new()
        .with_title("Conway's Game of Life")
        .with_inner_size(size)
        .build(&event_loop)?;

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let mut size = window_size;

    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

    let mut rng = thread_rng();

    let mut buffer1: CubeSphereGrid<bool, 200> = CubeSphereGrid::from_fn(|_| rng.gen());
    let mut buffer2: CubeSphereGrid<bool, 200> = CubeSphereGrid::default();

    event_loop.run(move |event, target| {
        match event {
            Event::NewEvents(StartCause::Init) => {
                target.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(1000 / 20)))
            },
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(window_size) => {
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
                        println!("Frame");

                        buffer2.set_from_neighbours_diagonals(&buffer1, |s1, s2, s3, s4, current, s6, s7, s8, s9| {
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

                        swap(&mut buffer2, &mut buffer1);

                        let frame = pixels.frame_mut();
                
                        for y in 0..size.height {
                            for x in 0..size.width {
                                let i = (y as usize * size.width as usize + x as usize) * 4;

                                let latitude = (y as f64 / size.height as f64) * PI - PI / 2.0;
                                let longitude = (x as f64 / size.width as f64) * PI * 2.0;

                                let value = buffer1[CubeSpherePoint::from_geographic(latitude, longitude)];

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
