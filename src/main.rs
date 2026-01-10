use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};
use minifb::{Window, WindowOptions};
use nalgebra::{Matrix4, Vector3};
use crate::{camera::{Camera, filter_primitive_3d, project_primitive}, drawing::{Primitive2i, draw_primitive, projected_primitive_to_screen_primitive}, objects::{Geometry, Object, pointclouds::PointCloud, surfaces::Surface}};

mod drawing;
mod objects;
mod camera;

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 800;
pub const EPSILON: f32 = 0.1;

fn main() {
    let mut window = Window::new("Manifolds-visualizer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("Echec lors de la création de fenêtre : {}", e));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut camera = Camera::new_looking_at_origin_from(FRAC_PI_3, 0.0, 0.0, 0.0, 1000.);

    //let mut s2 = Object::new(Geometry::Surface(Surface::new_sphere(10.0, 60, 30)), Matrix4::identity(), 0x00ff00);
    let mut t2 = Object::new(Geometry::Surface(Surface::new_torus(500.0, 300.0, 100, 50)), Matrix4::identity(), 0xff0000);

    let cloud = PointCloud::new_from_path("./scan.ply").unwrap();

    let mut p3 = Object::new(Geometry::PointCloud(cloud), Matrix4::identity(), 0x00ff00);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| panic!("Echec lors de l'actualisation du framebuffer : {}", e));

            buffer = vec![0; WIDTH * HEIGHT];

            object_to_screen_primitives(&p3, &camera, WIDTH, HEIGHT)
                .chain(object_to_screen_primitives(&t2, &camera, WIDTH, HEIGHT))
                .for_each(|primitive| draw_primitive(primitive, &mut buffer, WIDTH, HEIGHT));

            let speed = 20.;
            let angle_speed = 0.006;

            t2.rotate_x(angle_speed);
            t2.rotate_y(angle_speed);
            t2.rotate_z(angle_speed);

            if window.is_key_down(minifb::Key::W) {
                camera.translate_relative(Vector3::new(0.0, 0.0, speed));
            }
            if window.is_key_down(minifb::Key::S) {
                camera.translate_relative(Vector3::new(0.0, 0.0, -speed));
            }
            if window.is_key_down(minifb::Key::A) {
                camera.translate_relative(Vector3::new(-speed, 0.0, 0.0));
            }
            if window.is_key_down(minifb::Key::D) {
                camera.translate_relative(Vector3::new(speed, 0.0, 0.0));
            }
            if window.is_key_down(minifb::Key::Up) {
                camera.rotate_pitch(-angle_speed);
            }
            if window.is_key_down(minifb::Key::Down) {
                camera.rotate_pitch(angle_speed);
            }
            if window.is_key_down(minifb::Key::Left) {
                camera.rotate_yaw(angle_speed);
            }
            if window.is_key_down(minifb::Key::Right) {
                camera.rotate_yaw(-angle_speed);
            }
    }
}

//le pipeline de rendu pour un objet
fn object_to_screen_primitives<'a>(object: &Object, camera: &Camera, width: usize, height: usize) -> impl Iterator<Item = Primitive2i> {
    let perspective_center_distance = camera.perspective_center_distance;
    let world_primitives = object.primitives();
    let camera_primitives = world_primitives.map(move |primitive| camera.world_primitive_to_camera_coordinates(primitive));
    let camera_visible_primitives = camera_primitives.flat_map(filter_primitive_3d);
    let camera_projected_primitives = camera_visible_primitives.map(move |primitive| project_primitive(primitive, perspective_center_distance));
    camera_projected_primitives.map(move |projected| projected_primitive_to_screen_primitive(projected, width, height))
}
