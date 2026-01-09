use std::f32::consts::{FRAC_PI_3, FRAC_PI_4};
use minifb::{Window, WindowOptions};
use nalgebra::Vector3;
use crate::{camera::{Camera, adjust_segment_3d, project_segment, projected_to_pixel}, drawing::draw_line, surfaces::SurfaceParam};

mod drawing;
mod surfaces;
mod camera;

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 800;
pub const EPSILON: f32 = 0.1;

fn main() {
    let mut window = Window::new(
        "Manifold-visualizer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("Echec lors de la création de fenêtre : {}", e));

    let aspect = WIDTH as f32 / HEIGHT as f32;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let s2 = SurfaceParam::new_torus(10.0, 4.0);
    let mut camera = Camera::new_looking_at_origin_from(FRAC_PI_3, 0.0, 0.0, 0.0, 25.0);

    let vertices: Vec<_> = s2.isovertices(70, 35).collect();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| panic!("Echec lors de l'actualisation du framebuffer : {}", e));

            let vertices_for_camera = vertices.iter().map(|points| camera.world_segment_to_camera_coordinates(*points));
            let visible = vertices_for_camera.flat_map(adjust_segment_3d);
            let projected = visible.map(|points|project_segment(points, camera.perspective_center_distance));
            let on_pixels_plane: Vec<_> = projected.map(|(p0, p1)| {
            let p0_pixels = projected_to_pixel(p0.0, p0.1, WIDTH as i32, HEIGHT as i32, aspect);
            let p1_pixels = projected_to_pixel(p1.0, p1.1, WIDTH as i32, HEIGHT as i32, aspect);
        
                (p0_pixels, p1_pixels)
            }).collect();
        
            buffer = vec![0; WIDTH * HEIGHT];
            for (p0, p1) in on_pixels_plane {
                draw_line(&mut buffer, WIDTH as i32, HEIGHT as i32, p0.0, p0.1, p1.0, p1.1, 0x00ff00);
            }

            let speed = 0.5;
            let angle_speed = 0.002;

            if window.is_key_down(minifb::Key::W) {
                camera.translate_relative(Vector3::new(0.0, 0.0, -speed));
            }
            if window.is_key_down(minifb::Key::S) {
                camera.translate_relative(Vector3::new(0.0, 0.0, speed));
            }
            if window.is_key_down(minifb::Key::A) {
                camera.translate_relative(Vector3::new(speed, 0.0, 0.0));
            }
            if window.is_key_down(minifb::Key::D) {
                camera.translate_relative(Vector3::new(-speed, 0.0, 0.0));
            }
            if window.is_key_down(minifb::Key::Up) {
                camera.rotate_pitch(angle_speed);
            }
            if window.is_key_down(minifb::Key::Down) {
                camera.rotate_pitch(-angle_speed);
            }
            if window.is_key_down(minifb::Key::Left) {
                camera.rotate_yaw(-angle_speed);
            }
            if window.is_key_down(minifb::Key::Right) {
                camera.rotate_yaw(angle_speed);
            }
    }
}
