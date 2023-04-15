//#![feature(dyn_star)]
//#![feature(pointer_sized_trait)]
extern crate sdl2;

pub(crate) mod scenes;
pub(crate) mod geometry;

use scenes::{Scene, Static, rotating_cube::*};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut scene: Box<dyn Scene> = Box::new(Static{});

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown{ keycode: Some(keycode), ..}  => {
                    let mut has_scene = true;
                    match keycode {
                        Keycode::Num0 => scene= Box::new(Static{}),
                        Keycode::Num1 => scene= RotatingCube::new(),
                        _ => {has_scene=false;}
                    }
                    if has_scene {
                        scene.on_construction();
                    }
                },
                _ => {}
            }
        }

        scene.on_tick(&mut canvas, 0);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
