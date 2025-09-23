extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
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
    // canvas.draw_line(start, end)
    // canvas.clear();
    // canvas.present();

     loop {
        // canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
