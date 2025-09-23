use  sdl2::rect::Point;
pub struct points(pub Point, pub Point);
pub fn generate_points(width: i32, height: i32) -> Vec<points> {
    let mut res = Vec::new();
    res.push(points(Point::new((width - 200) / 2, 0), Point::new((width - 200) / 2, (height - 200) / 2)));
    res.push(points(Point::new(0, (height - 200) / 2), Point::new((width - 200) / 2, (height - 200) / 2)));
    res.push(points(Point::new(0, (height + 200) / 2), Point::new((height - 200) / 2,(width + 200) / 2)));
    res.push(points(Point::new((width- 200) / 2 , height), Point::new((height - 200) / 2,(width + 200) / 2)));
    // nas lakho
    res.push(points(Point::new((width + 200) / 2, 0), Point::new((width + 200) / 2, (height - 200) / 2)));
    res.push(points(Point::new(width, (height - 200) / 2), Point::new((width + 200) / 2, (height - 200) / 2)));
    res.push(points(Point::new((width + 200) / 2, height), Point::new((width + 200) / 2, (width + 200) / 2)));
    res.push(points(Point::new(width, (height + 200) / 2), Point::new((width + 200) / 2, (width + 200) / 2)));

    res
}