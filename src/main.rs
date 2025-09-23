extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let _ = canvas.draw_line(Point::new(300, 0), Point::new(300, 300));
    let _ = canvas.draw_line(Point::new(400, 0), Point::new(400, 800));
    let _ = canvas.draw_line(Point::new(500, 0), Point::new(500, 300));
    let _ = canvas.draw_line(Point::new(0, 300), Point::new(300, 300));
    let _ = canvas.draw_line(Point::new(0, 500), Point::new(300, 500));
    let _ = canvas.draw_line(Point::new(0, 400), Point::new(800, 400));
    let _ = canvas.draw_line(Point::new(300, 500), Point::new(300, 800));
    let _ = canvas.draw_line(Point::new(500, 500), Point::new(500, 800));
    let _ = canvas.draw_line(Point::new(500, 500), Point::new(800, 500));
    let _ = canvas.draw_line(Point::new(500, 300), Point::new(800, 300));




    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
