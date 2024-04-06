#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::{Duration, Instant};
use pixels::{PixelsBuilder, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{ElementState, Event, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow};
use winit::platform::scancode::PhysicalKeyExtScancode;
use textgraph::{Char, Font, Layer, pxy, xy, Grid, CellularMap, Dir, ToDirection, Fg, Color, Bg, WHITE, BLACK, BLUE, Sprite};

const WIN_SIZE: (u32, u32) = (640, 480);
const PIX_SIZE: (u32, u32) = (640, 480);

fn main() -> Result<(), EventLoopError> {
    let timer_length = Duration::from_millis(100); // do not make equal to 15
    let mut mouse_pos: (i32, i32) = (-1, -1);

    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop!");
    let window = winit::window::WindowBuilder::new()
        .with_title("437<3")
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
    let mut layer = Layer::new(&font, xy(80, 60), pxy(1, 1), pxy(0, 0));

    let mut rng = rand::SeedableRng::from_entropy();

    let now = Instant::now();
    let map = CellularMap::new(xy(80, 60)).build(&mut rng);
    let wall = Char('x' as u8) + Fg(Color::rgba(0xff, 0x99, 0x0, 0xff)) + Bg(BLACK);
    let empty = Char(' ' as u8) + Fg(WHITE) + Bg(BLACK);
    let mut player = Sprite {
        cell: Char('@' as u8) + Fg(WHITE) + Bg(BLUE),
        position: pxy(0, 0),
        scale: pxy(1, 1),
    };
    for pt in layer.size() {
        layer[pt] |= if map[pt] { wall } else { empty }
    }
    let elapsed = now.elapsed();
    println!("Mapgen time: {:.2?}", elapsed);

    let mut player_loc = map.find(|c| !*c).unwrap();
    player.position = layer.pixel_coord(player_loc);

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
                let start = Instant::now();
                draw(&mut pixels.frame_mut(), &layer, &player);
                pixels.render().unwrap();
                let dur = Instant::now() - start;
                println!("Drawn in {:.2?} ({} fps)", dur, 1000.0 / dur.as_millis() as f32)
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

            // Handle keyboard events
            Event::WindowEvent {
                window_id, event: WindowEvent::KeyboardInput { event, .. }
            } if window_id == window.id() => {
                if event.state.is_pressed() {
                    let dir: Option<Dir> = event.physical_key.to_scancode().to_direction();
                    if let Some(dir) = dir {
                        let new = player_loc.translate(dir);
                        if layer.contains(new) && layer[new] == empty {
                            player.position = layer.pixel_coord(new);
                            player_loc = new;
                            window.request_redraw();
                        }
                    }
                }
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

fn update(_layer: &mut Layer) {
}

fn draw(frame: &mut [u8], layer: &Layer, player: &Sprite) {
    frame.fill(0x0);
    layer.draw(frame, PIX_SIZE.0 as usize);
    layer.draw_sprites([player], frame, PIX_SIZE.0 as usize);
}
