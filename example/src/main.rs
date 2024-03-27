#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::{Duration, Instant};
use pixels::{PixelsBuilder, SurfaceTexture};
use rand::RngCore;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{ElementState, Event, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow};
use textgraph::{Char, Drawable, Font, Layer, pxy, xy, Grid, CountableNeighbors};

const WIN_SIZE: (u32, u32) = (640, 480);
const PIX_SIZE: (u32, u32) = (640, 480);

fn main() -> Result<(), EventLoopError> {
    let timer_length = Duration::from_millis(100); // do not make equal to 15
    let mut mouse_pos: (i32, i32) = (-1, -1);

    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop!");
    let window = winit::window::WindowBuilder::new()
        .with_title("The Thing")
        .with_inner_size(LogicalSize{ width: WIN_SIZE.0, height: WIN_SIZE.1 })
        .with_min_inner_size(LogicalSize { width: WIN_SIZE.0, height: WIN_SIZE.1 })
        .build(&event_loop)?;

    let mut pixels = {
        let PhysicalSize { width, height } = window.inner_size();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        PixelsBuilder::new(PIX_SIZE.0, PIX_SIZE.1, surface_texture)
            .build().expect("Failed to build pixels!")
    };

    let font = Font::default();
    let mut layer = Layer::new(&font, xy(80, 30), pxy(1, 2), pxy(0, 0));

    // Prime the layer with some living cells:
    let mut rng = rand::thread_rng();
    for _ in 0..800 {
        let pt = xy((rng.next_u32() % 80) as i32, (rng.next_u32() % 30) as i32);
        layer[pt.into()] |= Char('#' as u8);
    }

    event_loop.run(move |event, target| {
        match event {
            // Exit if we click the little x
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => { target.exit(); }

            // Redraw if it's redrawing time
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
            } if window_id == window.id() => {
                let now = Instant::now();
                draw(&mut pixels.frame_mut(), &layer);
                pixels.render().unwrap();
                let elapsed = now.elapsed();
                println!("Redraw time: {:.2?}", elapsed);
            }

            // Start the timer on init
            Event::NewEvents(StartCause::Init) => {
                target.set_control_flow(ControlFlow::WaitUntil(Instant::now() + timer_length));
            }

            // When the timer fires, update the world and restart the timer
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                update(&mut layer);
                window.request_redraw();
                target.set_control_flow(ControlFlow::WaitUntil(Instant::now() + timer_length));
            }

            // Update that the mouse moved if it did
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position: pos, device_id: _ },
                window_id
            } if window_id == window.id() => {
                let lp = pos.to_logical(window.scale_factor());
                mouse_pos = (lp.x, lp.y);
            }

            // Do something if the mouse was clicked
            Event::WindowEvent {
                window_id, event: WindowEvent::MouseInput { device_id: _, state: ElementState::Pressed, button: MouseButton::Left }
            } if window_id == window.id() => {
                // do nothing
                println!("Click {}, {}", mouse_pos.0, mouse_pos.1)
            }

            Event::WindowEvent {
                window_id, event: WindowEvent::Resized(new_size)
            } if window_id == window.id() => {
                println!("Resized to {}, {}", new_size.width, new_size.height);
                pixels.resize_surface(new_size.width, new_size.height).expect("Resize surface failure")
            }

            // Drop other events
            _ => {}
        }
    })
}

fn update(layer: &mut Layer) {
    let old_ch = layer.chars();
    for (pt, cell) in old_ch.iter().enumerate().map(|(n, c)| (old_ch.coord(n), c)) {
        let neighbor_count = old_ch.neighbors_equal(pt, Char('#' as u8)).count() +
            old_ch.diagonals_equal(pt, Char('#' as u8)).count();
        if (*cell == Char(' ' as u8) && neighbor_count == 3) ||
            (*cell == Char('#' as u8) && (neighbor_count == 2 || neighbor_count == 3)) {
            layer[pt] |= Char('#' as u8)
        } else {
            layer[pt] |= Char(' ' as u8)
        }
    }
}

fn draw(frame: &mut [u8], layer: &Layer) {
    frame.fill(0x0);
    layer.draw(frame, PIX_SIZE.0 as usize);
}
