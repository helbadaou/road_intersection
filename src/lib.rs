use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Points(pub Point, pub Point);
pub fn generate_points(size: (u32, u32)) -> Vec<Points> {
    let mut res: Vec<Points> = Vec::new();
    let middle_w: i32 = (size.0 / 2) as i32;
    let middle_h: i32 = (size.1 / 2) as i32;
    let road_w: i32 = ((size.0 * 12) / 100) as i32;
    let road_h: i32 = ((size.0 * 12) / 100) as i32;
    res.push(Points(
        Point::new(middle_w - road_w, 0),
        Point::new(middle_w - road_w, middle_h - road_h),
    ));
    res.push(Points(
        Point::new(0, middle_h - road_h),
        Point::new(middle_w - road_w, middle_h - road_h),
    ));
    res.push(Points(
        Point::new(middle_w + road_w, 0),
        Point::new(middle_w + road_w, middle_h - road_h),
    ));
    res.push(Points(
        Point::new(middle_w + road_w, middle_h - road_h),
        Point::new(middle_w * 2, middle_h - road_h),
    ));
    res.push(Points(
        Point::new(0, middle_h + road_h),
        Point::new(middle_w - road_w, middle_h + road_h),
    ));
    res.push(Points(
        Point::new(middle_w - road_w, middle_h + road_h),
        Point::new(middle_w - road_w, middle_h * 2),
    ));
    res.push(Points(
        Point::new(middle_w + road_w, middle_h + road_h),
        Point::new(middle_w * 2, middle_h + road_h),
    ));
    res.push(Points(
        Point::new(middle_w + road_w, middle_h + road_h),
        Point::new(middle_w + road_w, middle_h * 2),
    ));

    res.push(Points(
        Point::new(middle_w, 0),
        Point::new(middle_w, middle_h - road_h),
    ));
    res.push(Points(
        Point::new(middle_w, middle_h - road_h),
        Point::new(middle_w, middle_h - road_h),
    ));
    res.push(Points(
        Point::new(middle_w, middle_h + road_h),
        Point::new(middle_w, middle_h * 2),
    ));
    res.push(Points(
        Point::new(0, middle_h),
        Point::new(middle_w - road_w, middle_h),
    ));
    res.push(Points(
        Point::new(middle_w + road_w, middle_h),
        Point::new(middle_w * 2, middle_h),
    ));
    res
}
#[derive(Clone, Debug, Copy)]
pub enum Direction {
    Sud,  // buttom
    Nord, // top
    West, // right
    East, // left
}
#[derive(Clone, Debug, Copy)]
pub struct Traficlight {
    pub direction: Direction,
    pub color: Color,
    pub allowed: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub hight: u32,
}
impl Traficlight {
    pub fn new(
        direction: Direction,
        color: Color,
        allowed: bool,
        x: i32,
        y: i32,
        width: u32,
        hight: u32,
    ) -> Self {
        Self {
            direction,
            color,
            allowed,
            x,
            y,
            width,
            hight,
        }
    }
}
pub fn generate_traficlight(size: (u32, u32)) -> Vec<Traficlight> {
    let mut res = Vec::new();
    let middle_w: i32 = (size.0 / 2) as i32;
    let middle_h: i32 = (size.1 / 2) as i32;
    let road_w: i32 = ((size.0 * 12) / 100) as i32;
    let road_h: i32 = ((size.0 * 12) / 100) as i32;
    let width = (road_w / 2) as u32;
    let height = (road_h / 2) as u32;
    res.push(
        Traficlight::new(
            Direction::Sud,
            Color::RGB(200, 0, 0),
            false,
            middle_w + road_w + 2,
            middle_h + road_h + 2,
            width,
            height
        )
    );
    res.push(
        Traficlight::new(
            Direction::Nord,
            Color::RGB(200, 0, 0),
            false,
            middle_w - road_w - (width as i32) - 2,
            middle_h - road_h - (height as i32) - 1,
            width,
            height
        )
    );
    res.push(
        Traficlight::new(
            Direction::West,
            Color::RGB(200, 0, 0),
            false,
            middle_w + road_w + 3,
            middle_h - road_h - (height as i32) - 2,
            width,
            height
        )
    );
    res.push(
        Traficlight::new(
            Direction::East,
            Color::RGB(200, 0, 0),
            false,
            middle_w - road_w - (width as i32) - 2,
            middle_h + road_h + 2,
            width,
            height
        )
    );
    res
}
#[derive(PartialEq, Clone, Copy)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Debug, Copy)]
pub struct Car {
    pub dir: Direction,
    pub color: Color,
    pub passed: bool,
    pub i: i32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub hight: u32,
    pub is_turned: bool,
}
impl Car {
    pub fn new(
        dir: Direction,
        color: Color,
        i: i32,
        x: i32,
        y: i32,
        width: u32,
        hight: u32,
    ) -> Self {
        Self {
            dir,
            color,
            passed: false,
            i,
            x,
            y,
            width,
            hight,
            is_turned: false,
        }
    }
}

pub fn generate_cars(key_t: Key, size: (u32, u32)) -> Car {

    let middle_w: i32 = (size.0 / 2) as i32;
    let middle_h: i32 = (size.1 / 2) as i32;
    let road_w: i32 = ((size.0 * 12) / 100) as i32;
    let road_h: i32 = ((size.1 * 12) / 100) as i32;
    let width = (road_w / 2) as u32;
    let height = (road_h / 2) as u32;
    let color = vec![
        Color::RGB(255, 0, 0),
        Color::RGB(0, 255, 0),
        Color::RGB(0, 0, 255),
    ];

    let mut rng = rand::thread_rng();
    let  g = rng.gen_range(0..=2);

    match key_t {
        Key::Up => Car::new(
            Direction::Sud,
            color[g],
            g.try_into().unwrap(),
            middle_w + (width as i32) / 2,
            (size.1 as i32) - (height as i32),
            width,
            height,
        ),
        Key::Left => Car::new(
            Direction::West,
            color[g],
            g.try_into().unwrap(),
            (size.0 as i32) - (width as i32),
            middle_h - (height as i32) / 2 - (height as i32),
            width,
            height,
        ),
        Key::Down => Car::new(
            Direction::Nord,
            color[g],
            g.try_into().unwrap(),
            middle_w - (width as i32) / 2 - (height as i32),
            0,
            width,
            height,
        ),
        _ => Car::new(
            Direction::East,
            color[g],
            g.try_into().unwrap(),
            0,
            middle_h + (height as i32) / 2,
            width,
            height,
        ),
    }
}
