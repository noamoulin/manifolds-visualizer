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
    world_to_cam: Matrix4<f32>,
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
            world_to_cam,
        }
    }

    pub fn world_to_cam(&self) -> &Matrix4<f32> {
        &self.world_to_cam
    }

    //effectue une translation de la caméra dans le monde, selon un vecteur exprimé dans le repère du monde
    pub fn translate_absolute(&mut self, dp: Vector3<f32>) {
        let translation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, dp.x,
            0.0, 1.0, 0.0, dp.y,
            0.0, 0.0, 1.0, dp.z,
            0.0, 0.0, 0.0, 1.0
        );

        self.pos += dp;
        self.world_to_cam = self.world_to_cam * translation_matrix;
    }

    //effectue une translation de la camera dans le monde, selon un vecteur exprimé dans le repère de la caméra
    pub fn translate_relative(&mut self, dp: Vector3<f32>) {
        let translation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, dp.x,
            0.0, 1.0, 0.0, dp.y,
            0.0, 0.0, 1.0, dp.z,
            0.0, 0.0, 0.0, 1.0
        );

        self.world_to_cam = translation_matrix * self.world_to_cam;
        let cam_to_world_rot = self.world_to_cam.fixed_view::<3, 3>(0, 0).transpose();
        let abs_dp = cam_to_world_rot * dp;
        self.pos += abs_dp;
    }

    //rotation de la camera sur son vecteur "devant" (axe z de son repère)
    pub fn rotate_roll(&mut self, d_phi: f32) {
         let cos_phi = d_phi.cos();
         let sin_phi = d_phi.sin();

         let rotation_matrix = Matrix4::new(
            cos_phi, -sin_phi, 0.0, 0.0,
            sin_phi, cos_phi,  0.0, 0.0,
            0.0,     0.0,      1.0, 0.0,
            0.0,     0.0,      0.0, 1.0,
         );

         let rotation_only = rotation_matrix.fixed_view::<3, 3>(0, 0);

         self.up = rotation_only * self.up;
         self.right = rotation_only * self.right;

         self.world_to_cam = rotation_matrix * self.world_to_cam;
    }

    //rotation de la camera sur son vecteur "droite" (axe y de son repère)
    pub fn rotate_pitch(&mut self, d_theta: f32) {
        let cos_theta = d_theta.cos();
        let sin_theta = d_theta.sin();

        let rotation_matrix = Matrix4::new(
            cos_theta, 0.0, -sin_theta, 0.0,
            0.0,       1.0, 0.0,        0.0,
            sin_theta, 0.0, cos_theta,  0.0,
            0.0,       0.0, 0.0,        1.0
        );

        let rotation_only = rotation_matrix.fixed_view::<3, 3>(0, 0);

        self.forward = rotation_only * self.forward;
        self.up = rotation_only * self.up;

        self.world_to_cam = rotation_matrix * self.world_to_cam;
    }

    //rotation de la camera sur son vecteur "haut" (axe x de son repère)
    pub fn rotate_yaw(&mut self, d_psi: f32) {
        let cos_psi = d_psi.cos();
        let sin_psi = d_psi.sin();

        let rotation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, cos_psi, -sin_psi, 0.0,
            0.0, sin_psi, cos_psi, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let rotation_only = rotation_matrix.fixed_view::<3, 3>(0, 0);

        self.forward = rotation_only * self.forward;
        self.right = rotation_only * self.forward;

        self.world_to_cam = rotation_matrix * self.world_to_cam;
    }
}
