extern crate sdl2;
use rand::Rng;
use road_intersection::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{ Duration, Instant };

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
    let mut lights = generate_traficlight(canvas.window().size());
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars: Vec<Vec<Car>> = vec![vec![], vec![], vec![], vec![]];

    let mut current_light = 0;
    let mut light_timer = Instant::now();
    let green_light_duration = Duration::from_secs(10);

    'running: loop {
        let now = Instant::now();

        if now.duration_since(light_timer) >= green_light_duration {
            current_light = (current_light + 1) % 4;
            light_timer = now;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }

                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let vehicles_not_passed: Vec<Car> = cars[0]
                        .iter()
                        .filter(|car| !car.passed)
                        .cloned()
                        .collect();
                    if vehicles_not_passed.len() == 0 || cars[0].len() == 0 {
                        cars[0].push(generate_cars(Key::Up, canvas.window().size()));
                    } else {
                        let car = &vehicles_not_passed[vehicles_not_passed.len() - 1];
                        let car1 = generate_cars(Key::Up, canvas.window().size());
                        if car1.y - car.y > (car1.hight as i32) + 20 {
                            cars[0].push(car1);
                        }
                    }
                }

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let vehicles_not_passed: Vec<Car> = cars[1]
                        .iter()
                        .filter(|car| !car.passed)
                        .cloned()
                        .collect();
                    if vehicles_not_passed.len() == 0 || cars[1].len() == 0 {
                        cars[1].push(generate_cars(Key::Down, canvas.window().size()));
                    } else {
                        let car = &vehicles_not_passed[vehicles_not_passed.len() - 1];
                        let car1 = generate_cars(Key::Down, canvas.window().size());
                        if car.y > (car1.hight as i32) + 20 {
                            cars[1].push(car1);
                        }
                    }
                }

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let vehicles_not_passed: Vec<Car> = cars[2]
                        .iter()
                        .filter(|car| !car.passed)
                        .cloned()
                        .collect();
                    if vehicles_not_passed.len() == 0 || cars[2].len() == 0 {
                        cars[2].push(generate_cars(Key::Left, canvas.window().size()));
                    } else {
                        let car = &vehicles_not_passed[vehicles_not_passed.len() - 1];
                        let car1 = generate_cars(Key::Left, canvas.window().size());
                        if car1.x - car.x > (car1.hight as i32) + 20 {
                            cars[2].push(car1);
                        }
                    }
                }

                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let vehicles_not_passed: Vec<Car> = cars[3]
                        .iter()
                        .filter(|car| !car.passed)
                        .cloned()
                        .collect();

                    if cars[3].len() == 0 || vehicles_not_passed.len() == 0 {
                        cars[3].push(generate_cars(Key::Right, canvas.window().size()));
                    } else {
                        let car = &vehicles_not_passed[vehicles_not_passed.len() - 1];
                        let car1 = generate_cars(Key::Right, canvas.window().size());
                        if car.x > (car1.hight as i32) + 20 {
                            cars[3].push(car1);
                        }
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let rend = vec![Key::Up, Key::Down, Key::Left, Key::Right];
                    let mut rng = rand::thread_rng();
                    let g = rng.gen_range(0..=3);
                    if cars[g].len() == 0 {
                        cars[g].push(generate_cars(rend[g], canvas.window().size()));
                    } else {
                        let vehicles_not_passed: Vec<Car> = cars[g]
                            .iter()
                            .filter(|car| !car.passed)
                            .cloned()
                            .collect();
                        if vehicles_not_passed.len() > 0 {
                            let car = &vehicles_not_passed[vehicles_not_passed.len() - 1];
                            let car1 = generate_cars(rend[g], canvas.window().size());
                            if
                                (g == 3 && car.x > (car1.hight as i32) + 20) ||
                                (g == 0 && car1.y - car.y > (car1.hight as i32) + 20) ||
                                (g == 1 && car.y > (car1.hight as i32) + 20) ||
                                (g == 2 && car1.x - car.x > (car1.hight as i32) + 20)
                            {
                                cars[g].push(car1);
                            }
                        }
                    }
                }
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

        for i in 0..cars.len() {
            for j in 0..cars[i as usize].len() {
                if cars[i][j].passed {
                    match cars[i][j].dir {
                        Direction::Sud => {
                            cars[i][j].y -= 1;
                            if cars[i][j].y == 400 + 30 && !cars[i][j].is_turned {
                                cars[i][j].is_turned = true;
                                cars[i][j].dir = Direction::Sud;
                            }
                        }
                        Direction::Nord => {
                            cars[i][j].y += 1;
                            if
                                cars[i][j].y == 400 - 30 - (cars[i][j].hight as i32) &&
                                !cars[i][j].is_turned
                            {
                                cars[i][j].is_turned = true;
                                cars[i][j].dir = Direction::West;
                            }
                        }
                        Direction::West => {
                            cars[i][j].x -= 1;
                            if cars[i][j].x == 400 + 30 && !cars[i][j].is_turned {
                                cars[i][j].is_turned = true;
                                cars[i][j].dir = match cars[i][j].i {
                                    0 => Direction::West,
                                    1 => Direction::Nord,
                                    _ => Direction::Sud,
                                };
                            }
                        }
                        Direction::East => {
                            cars[i][j].x += 1;
                            if
                                cars[i][j].x == 400 - 30 - (cars[i][j].hight as i32) &&
                                !cars[i][j].is_turned &&
                                cars[i][j].i != 3
                            {
                                cars[i][j].is_turned = true;
                                cars[i][j].dir = match cars[i][j].i {
                                    0 => Direction::East,
                                    1 => Direction::Nord,
                                    _ => Direction::Sud,
                                };
                            } else if
                                cars[i][j].i == 3 &&
                                cars[i][j].x == 400 - 30 - (cars[i][j].hight as i32) * 3 &&
                                !cars[i][j].is_turned
                            {
                                cars[i][j].is_turned = true;
                                cars[i][j].dir = Direction::Sud;
                            }
                        }
                    }
                    continue;
                }

                let can_move = i == current_light;

                match i {
                    0 => {
                        if j == 0 && !cars[i][j].passed {
                            if  cars[i][j].y > 500 || can_move {
                                cars[i][j].y -= 1;
                            }
                            if cars[i][j].y <= 500 {
                                cars[i][j].passed = true;
                            }
                        }
                        if j != 0 && !cars[i][j].passed {
                            if can_move {
                                cars[i][j].y -= 1;
                            }

                        }
                    }
                    1 => {
                        if j == 0 && !cars[i][j].passed {
                            if cars[i][j].y <= 300 ||can_move {
                                cars[i][j].y += 1;
                            }
                            if cars[i][j].y >= 300 {
                                cars[i][j].passed = true;
                            }
                        } else if j != 0 && !cars[i][j].passed {
                            if  cars[i][j].y >= 300 || can_move {
                                cars[i][j].y += 1;
                            }
                            if cars[i][j].y >= 300 {
                                cars[i][j].passed = true;
                            }
                        }
                    }
                    2 => {
                        if j == 0 && !cars[i][j].passed {
                            if can_move {
                                cars[i][j].x -= 1;
                            }
                            if cars[i][j].x <= 400 {
                                cars[i][j].passed = true;
                            }
                        } else if j != 0 && !cars[i][j].passed {
                            if can_move {
                                cars[i][j].x -= 1;
                            }
                            if cars[i][j].x <= 400 {
                                cars[i][j].passed = true;
                            }
                        }
                    }
                    3 => {
                        if j == 0 && !cars[i][j].passed {
                            if can_move {
                                cars[i][j].x += 1;
                            }
                            if cars[i][j].x >= 400 {
                                cars[i][j].passed = true;
                            }
                        } else if j != 0 && !cars[i][j].passed {
                            if can_move {
                                cars[i][j].x += 1;
                            }
                            if cars[i][j].x >= 400 {
                                cars[i][j].passed = true;
                            }
                        }
                    }
                    _ => {}
                }
            }

            for car in cars[i].iter() {
                canvas.set_draw_color(car.color);
                let _ = canvas.fill_rect(Rect::new(car.x, car.y, car.width, car.hight));
            }
        }

        for (i, light) in lights.iter_mut().enumerate() {
            if i == current_light {
                light.color = Color::RGB(0, 255, 0);
                light.allowed = true;

                for car in cars[current_light].iter_mut() {
                    if !car.passed {
                        if
                            (current_light == 0 && car.y <= 400 + 30 + 5) ||
                            (current_light == 1 && car.y + (car.hight as i32) >= 400 - 30 - 5) ||
                            (current_light == 2 && car.x <= 400 + 30 + 5) ||
                            (current_light == 3 && car.x + (car.width as i32) >= 400 - 30 - 5)
                        {
                            car.passed = true;
                        }
                    }
                }
            } else {
                light.allowed = false;
                light.color = Color::RGB(255, 0, 0);
            }

            canvas.set_draw_color(light.color);
            let _ = canvas.fill_rect(Rect::new(light.x, light.y, light.width, light.hight));
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
