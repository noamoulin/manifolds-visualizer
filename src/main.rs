use minifb::{Window, WindowOptions};
use nalgebra::{Matrix4, Perspective3, Vector3};

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
    fovy: f32,
    perspective_center_distance: f32,
    world_to_cam: Matrix4<f32>,
}

impl Camera {
    //camera à (0, O, 0) regardant vers (0, 0, -1), up vers (0, 1, 0)
    pub fn new_at_origin(fovy: f32) -> Self {
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

        let perspective_center_distance = 1.0 / (fovy / 2.0).tan();

        Self {
            pos,
            forward,
            up,
            right,
            fovy,
            world_to_cam,
            perspective_center_distance,
        }
    }

    pub fn new_looking_at_origin_from(fovy: f32, init_droll: f32, init_dpitch: f32, init_dyaw: f32, distance: f32) -> Self {
        let mut camera = Self::new_at_origin(fovy);

        camera.rotate_roll(init_droll);
        camera.rotate_pitch(init_dpitch);
        camera.rotate_yaw(init_dyaw);
        camera.translate_relative(Vector3::new(0.0, 0.0, -distance));

        camera
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

//fait l'hypothèse que le segment et projetable dans son intégralité (devant le plan xy) et exprimé dans le repère de la caméra
fn project_segment(points: (Vector3<f32>, Vector3<f32>), perspective_center_distance: f32) -> ((f32, f32), (f32, f32)) {
    (project_point(points.0, perspective_center_distance), project_point(points.1, perspective_center_distance))
}

//fait l'hypothèse que le point est projetable (devant xy) et exprimé dans le repère de la caméra
fn project_point(point: Vector3<f32>, perspective_center_distance: f32) -> (f32, f32) {
    let point_distance_to_center = perspective_center_distance + point.z;

    let dxdz = point.x / point_distance_to_center;
    let dydz = point.y / point_distance_to_center;

    let dx_proj = point.x + dxdz * point.z;
    let dy_proj = point.y + dydz * point.z;

    (point.x + dx_proj, point.y + dy_proj)
}

fn projected_to_pixel(x: f32, y: f32, width: i32, height: i32, aspect: f32) -> (i32, i32) {
    let u = (x + aspect) / (2.0 * aspect);
    let v = (1.0 - y) / 2.0;

    let i = (u * (width as f32 - 1.0)).round() as i32;
    let j = (v * (height as f32 - 1.0)).round() as i32;

    (i, j)
}

//pour supprimer les segments situés derrière le plan de la camera (si un sengement coupe le plan, l'adapte pour qu'il soit projetable)
fn adjust_segment_3d(points: (Vector3<f32>, Vector3<f32>)) -> Option<(Vector3<f32>, Vector3<f32>)> {
    match (points.0.z >= 0.0, points.1.z >= 0.0) {
        (true, true) => Some(points),
        (true, false) => Some(adjusted_segment_3d(points.0, points.1)),
        (false, true) => Some(adjusted_segment_3d(points.1, points.0)),
        (false, false) => None,
    }
}

//pour un segment donc p0 est situé devant le plan xy (z >= 0) et p1 derriere, tranlate p1 sur le segment de manière à le positionner sur xy
fn adjusted_segment_3d(p0: Vector3<f32>, p1: Vector3<f32>) -> (Vector3<f32>, Vector3<f32>) {
    let dxdz = (p1.x - p0.x) / (p1.z - p0.z);
    let dydz = (p1.y - p0.y) / (p1.z - p0.z);

    let dx_intersection = dxdz * -p0.z;
    let dy_intersection = dydz * -p0.z;

    (p0, Vector3::new(p0.x + dx_intersection, p0.y + dy_intersection, 0.0))
}
