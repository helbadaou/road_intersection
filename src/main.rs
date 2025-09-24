extern crate sdl2;
use road_intersection::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::time::Duration;

pub struct Sud {}
pub struct Nord {}
pub struct Est {}
pub struct East {}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let window_height = 800;
    // let window_width = 800;
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
                    if cars[0].len() == 0 {
                        cars[0].push(generate_cars(Key::Up, canvas.window().size()));
                    } else {
                        let vehicles_not_passed: Vec<Car> = cars[0].iter().filter(|car| !car.passed).cloned().collect();
                        let car = &vehicles_not_passed[vehicles_not_passed.len()-1];
                        let car1  = generate_cars(Key::Up, canvas.window().size()) ;
                        if car1.y - car.y > car1.hight as i32 + 20 {
                            cars[0].push(car1);
                        } 
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if cars[1].len() == 0 {
                        cars[1].push(generate_cars(Key::Down, canvas.window().size()));
                    } else {
                        let vehicles_not_passed: Vec<Car> = cars[1].iter().filter(|car| !car.passed).cloned().collect();
                        let car = &vehicles_not_passed[vehicles_not_passed.len()-1];
                        let car1  = generate_cars(Key::Down, canvas.window().size());
                        if car.y > car1.hight as i32 + 20 {
                            cars[1].push(car1);
                        }
                    }
                },

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if cars[2].len() == 0 {
                        cars[2].push(generate_cars(Key::Left, canvas.window().size()));
                    } else {
                        let vehicles_not_passed: Vec<Car> = cars[2].iter().filter(|car| !car.passed).cloned().collect();
                        let car = &vehicles_not_passed[vehicles_not_passed.len()-1];
                        let car1  = generate_cars(Key::Left, canvas.window().size());
                        if car1.x - car.x > car1.hight as i32 + 20 {
                            cars[2].push(car1);
                        } 
                    }
                },

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if cars[3].len() == 0 {
                        cars[3].push(generate_cars(Key::Right, canvas.window().size()));
                    } else {
                        let vehicles_not_passed: Vec<Car> = cars[3].iter().filter(|car| !car.passed).cloned().collect();
                        let car = &vehicles_not_passed[vehicles_not_passed.len()-1];
                        let car1  = generate_cars(Key::Right, canvas.window().size());
                        if car.x > car1.hight as i32 + 20 {
                            cars[3].push(car1);
                        } 
                    }
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



        for carss in cars.iter_mut() {
            for car in carss.iter_mut() {
                match car.dir {
                    Direction::Sud => {
                        car.y -= 1;
                        if car.y == 400 + 24 && !car.is_turned {
                            car.is_turned = true;
                            car.dir = Direction::East;
                        }
                    }
                    Direction::Nord => {
                        car.y += 1;
                        if car.y == 400 - 24 - car.hight as i32 && !car.is_turned {
                            car.is_turned = true;
                            car.dir = Direction::West;
                        }
                    }
                    Direction::West => {
                        car.x -= 1;
                        if car.x == 400 + 24 && !car.is_turned {
                            car.is_turned = true;
                            car.dir = Direction::Sud;
                        }
                    }
                    Direction::East => {
                        car.x += 1;
                        if car.x == 400 - 24 - car.hight as i32 && !car.is_turned {
                            car.is_turned = true;
                            car.dir = Direction::Nord;
                        }
                    }
                }
            }
            for car in carss.iter() {
                canvas.set_draw_color(car.color);
                let _ = canvas.fill_rect(Rect::new(car.x, car.y, car.width, car.hight));
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
