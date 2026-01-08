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
    //println!("{:?}, {:?}", x1, x0);
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        // Écrire dans le buffer seulement si valide
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
