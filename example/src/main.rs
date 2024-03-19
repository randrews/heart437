#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::{Duration, Instant};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{ElementState, Event, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use textgraph::{BLUE, Canvas, CharSize, Color, Drawable, Font, Layer, PixelSize, RectStyle, WHITE};

const WIN_SIZE: (u32, u32) = (640, 480);
const PIX_SIZE: (u32, u32) = (640, 480);

fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    let timer_length = Duration::from_millis(15);
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
            .clear_color(wgpu::Color{ r: 0.1, g: 0.1, b: 0.15, a: 1.0 })
            .build().expect("Failed to build pixels!")
    };

    let mut offset = 0;
    let font = Font::default();
    let mut layer = Layer::new(&font, CharSize(80, 30), PixelSize(2, 4), PixelSize(0, 0));
    layer.fill(Some('R'), Some(WHITE), Some(Color::rgba(0, 0, 0, 64)));
    layer.rect(RectStyle::DOUBLE.wall(), Some(WHITE), Some(BLUE), CharSize(1, 1), CharSize(5, 3));

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
                layer.set_origin(PixelSize(offset, offset));
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
                offset = offset + 1;
                if offset > 100 { offset = 0 }
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

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw(frame: &mut [u8], layer: &Layer) {
    frame.fill(0x0);
    layer.draw(frame, PIX_SIZE.0 as usize);
}
