use derive_more::Constructor;

use crate::camera::{Point2f, Primitive2f};


//primitive dessinable sur l'écran
pub enum Primitive2i{
    Line(Line2i),
    Point(Point2i),
}

#[derive(Constructor)]
pub struct Line2i {
    p0: Point2i,
    p1: Point2i,
}

#[derive(Constructor)]
pub struct Point2i {
    p: (i32, i32),
    color: u32,
}

pub fn projected_to_pixel(point: Point2f, width: i32, height: i32) -> Point2i {
    let aspect = width as f32 / height as f32;
    let u = (point.p.0 + aspect) / (2.0 * aspect);
    let v = (1.0 - point.p.1) / 2.0;

    let i = (u * (width as f32 - 1.0)).round() as i32;
    let j = (v * (height as f32 - 1.0)).round() as i32;

    Point2i::new((i, j), point.color)
}

pub fn projected_primitive_to_screen_primitive(projected: Primitive2f, width: usize, height: usize) -> Primitive2i {
    match projected {
        Primitive2f::Line(line) => {
            let p0_pixel = projected_to_pixel(line.p0, width as i32, height as i32);
            let p1_pixel = projected_to_pixel(line.p1, width as i32, height as i32);

            Primitive2i::Line(Line2i::new(p0_pixel, p1_pixel))
        }
        Primitive2f::Point(point) => Primitive2i::Point(projected_to_pixel(point, width as i32, height as i32))
    }
}

pub fn draw_line(
    buffer: &mut [u32],
    width: i32,
    height: i32,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: u32,
) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        if 0 <= x0 && x0 < width && 0 <= y0 && y0 < height {
            let idx = (y0 as usize) * (width as usize) + (x0 as usize);
            buffer[idx] = color;
        }

        if x0 == x1 && y0 == y1 { break; }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_point(buffer: &mut [u32], point: &Point2i, width: i32, height: i32) {
    if point.p.0 >= 0 && point.p.0 < width && point.p.1 >=0 && point.p.1 < height {
        let idx = (point.p.1 as usize) * (width as usize) + (point.p.0 as usize);

        if buffer[idx] == 0 {
            buffer[idx] = point.color;
        }
    }
}

pub fn draw_primitive(primitive: &Primitive2i, buffer: &mut [u32], width: usize, height: usize) {
    match primitive {
        Primitive2i::Line(line) => draw_line(buffer, width as i32, height as i32, line.p0.p.0, line.p0.p.1, line.p1.p.0, line.p1.p.1, line.p0.color),
        Primitive2i::Point(point) => draw_point(buffer, point, width as i32, height as i32),
    }
}