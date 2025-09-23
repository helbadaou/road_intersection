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
    // // nas lakho
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
        Point::new(middle_w, middle_h *2),
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
