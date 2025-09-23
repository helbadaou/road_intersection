extern crate sdl2;
use road_intersection::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 600, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let lines = generate_points(canvas.window().size());
    
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
    

    canvas.set_draw_color(Color::RGB(0, 0, 0)); 
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for i in &lines {
        let _ = canvas.draw_line(i.0, i.1);
    }

    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

}
