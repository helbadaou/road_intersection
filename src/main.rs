extern crate sdl2;
use road_intersection::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub struct Sud {}
pub struct Nord {}
pub struct Est {}
pub struct Ouest {}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let lines = generate_points(canvas.window().size());
    let lights = generate_traficlight(canvas.window().size());

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars: Vec<Vec<Car>> = vec![vec![], vec![], vec![], vec![]];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    cars[0].push(generate_cars(Key::Up, canvas.window().size()));
                    let vehicles_not_passed: Vec<Car> =
                        cars[0].iter().filter(|car| !car.passed).cloned().collect();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    cars[1].push(generate_cars(Key::Down, canvas.window().size()));
                    let vehicles_not_passed: Vec<Car> =
                        cars[1].iter().filter(|car| !car.passed).cloned().collect();
                },

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    cars[2].push(generate_cars(Key::Left, canvas.window().size()));
                    let vehicles_not_passed: Vec<Car> =
                        cars[2].iter().filter(|car| !car.passed).cloned().collect();
                },

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    cars[3].push(generate_cars(Key::Right, canvas.window().size()));
                    let vehicles_not_passed: Vec<Car> =
                        cars[3].iter().filter(|car| !car.passed).cloned().collect();
                },

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for (h, i) in lines.iter().enumerate() {
            if h > 7 {
                canvas.set_draw_color(Color::RGB(128, 128, 128));
            } else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            let _ = canvas.draw_line(i.0, i.1);
        }

        for light in lights.iter() {
            canvas.set_draw_color(light.color);
            let _ = canvas.draw_rect(Rect::new(light.x, light.y, light.width, light.hight));
        }
        // if cars.len() > 0 {
        for car in cars[0].iter_mut() {
            match car.dir {
                Direction::Sud => {
                    car.y -= 1;
                }
                Direction::Nord => {
                    car.y += 1;
                }
                Direction::West => {
                    car.x -= 1;
                }
                Direction::Ouest => {
                    car.x += 1;
                }
            }
        }
        for car in cars[0].iter() {
            canvas.set_draw_color(car.color);
            let _ = canvas.draw_rect(Rect::new(car.x, car.y, car.width, car.hight));
        }
        // }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
