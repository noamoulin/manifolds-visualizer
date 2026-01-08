use minifb::{Window, WindowOptions};
use nalgebra::{Matrix4, Vector3};

mod drawing;
mod surfaces;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

fn main() {
    let mut window = Window::new(
        "Manifold-visualizer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("Echec lors de la création de fenêtre : {}", e));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| panic!("Echec lors de l'actualisation du framebuffer : {}", e));
    }
}

pub struct Camera {
    pos: Vector3<f32>,
    forward: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,
    fov: f32,
    world_to_cam: Option<Matrix4<f32>>,
}

impl Camera {
    //camera à (0, O, 0) regardant vers (0, 0, -1), up vers (0, 1, 0)
    pub fn new_at_origin(fov: f32) -> Self {
        let pos = Vector3::new(0.0, 0.0, 0.0);
        let forward = Vector3::new(0.0, 0.0, -1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let right = forward.cross(&up);

        let world_to_cam = Matrix4::new(
            right.x, up.x, forward.x, 0.0,
            right.y, up.y, forward.y, 0.0, 
            right.z, up.z, forward.z, 0.0,
            0.0,     0.0,  0.0,       1.0,
        );

        Self {
            pos,
            forward,
            up,
            right,
            fov,
            world_to_cam: Some(world_to_cam),
        }
    }
}
