use derive_more::Constructor;
use nalgebra::{Matrix4, Vector3};
use crate::{EPSILON, objects::{Line3f, Point3f, Primitive3f}};

pub struct Camera {
    pub perspective_center_distance: f32,
    world_to_cam: Matrix4<f32>,
    cam_to_world: Matrix4<f32>,
}

pub enum Primitive2f {
    Line(Line2f),
    Point(Point2f),
}

#[derive(Constructor)]
pub struct Line2f {
    pub p0: Point2f,
    pub p1: Point2f,
}

#[derive(Constructor)]
pub struct Point2f {
    pub p: (f32, f32),
    pub color: u32,
}

impl Camera {
    pub fn new_at_origin(fovy: f32) -> Self {
        let cam_to_world = Matrix4::identity();
        let world_to_cam = Matrix4::identity();

        let perspective_center_distance = 1.0 / (fovy / 2.0).tan();

        Self {
            world_to_cam,
            cam_to_world,
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

    fn update_world_to_cam(&mut self) {
        self.world_to_cam = self.cam_to_world.try_inverse().unwrap();
    }

    //effectue une translation de la caméra dans le monde, selon un vecteur exprimé dans le repère du monde
    pub fn translate_absolute(&mut self, dp: Vector3<f32>) {
        let translation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, dp.x,
            0.0, 1.0, 0.0, dp.y,
            0.0, 0.0, 1.0, dp.z,
            0.0, 0.0, 0.0, 1.0
        );

        self.cam_to_world = translation_matrix * self.cam_to_world;
        self.update_world_to_cam();
    }

    //effectue une translation de la camera dans le monde, selon un vecteur exprimé dans le repère de la caméra
    pub fn translate_relative(&mut self, dp: Vector3<f32>) {
        let translation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, dp.x,
            0.0, 1.0, 0.0, dp.y,
            0.0, 0.0, 1.0, dp.z,
            0.0, 0.0, 0.0, 1.0
        );

        self.cam_to_world = self.cam_to_world * translation_matrix;
        self.update_world_to_cam();
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

         self.cam_to_world = self.cam_to_world * rotation_matrix;
         self.update_world_to_cam();
    }

    //rotation de la camera sur son vecteur "droite" (axe x de son repère)
    pub fn rotate_pitch(&mut self, d_theta: f32) {
        let cos_theta = d_theta.cos();
        let sin_theta = d_theta.sin();

        let rotation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, cos_theta, -sin_theta, 0.0,
            0.0, sin_theta, cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        self.cam_to_world = self.cam_to_world * rotation_matrix;
        self.update_world_to_cam();
    }

    //rotation de la camera sur son vecteur "haut" (axe y de son repère)
    pub fn rotate_yaw(&mut self, d_psi: f32) {
        let cos_psi = d_psi.cos();
        let sin_psi = d_psi.sin();

        let rotation_matrix = Matrix4::new(
            cos_psi, 0.0, -sin_psi, 0.0,
            0.0,     1.0, 0.0,      0.0,
            sin_psi, 0.0, cos_psi,  0.0,
            0.0,     0.0, 0.0,      1.0
        );

        self.cam_to_world = self.cam_to_world * rotation_matrix;
        self.update_world_to_cam();
    }

    pub fn world_line_to_camera_coordinates(&self, line: &Line3f) -> Line3f {
        Line3f::new(self.world_point_to_camera_coordinates(&line.p0), self.world_point_to_camera_coordinates(&line.p1))
    }

    fn world_point_to_camera_coordinates(&self, point: &Point3f) -> Point3f {
        let mut homogeneous = point.p.to_homogeneous();
        homogeneous.w = 1.0;
        let from_cam = self.world_to_cam * homogeneous;

        Point3f::new(Vector3::new(from_cam.x, from_cam.y, from_cam.z), point.color)
    }

    pub fn world_primitive_to_camera_coordinates(&self, primitive: &Primitive3f) -> Primitive3f {
        match primitive {
            Primitive3f::Line(line) => Primitive3f::Line(self.world_line_to_camera_coordinates(line)),
            Primitive3f::Point(point) => Primitive3f::Point(self.world_point_to_camera_coordinates(point)),
        }
    }
}

pub fn project_primitive(primitive: Primitive3f, perspective_center_distance: f32) -> Primitive2f {
    match primitive {
        Primitive3f::Line(line) => Primitive2f::Line(project_line(line, perspective_center_distance)),
        Primitive3f::Point(point) => Primitive2f::Point(project_point(point, perspective_center_distance)),
    }
}

//fait l'hypothèse que le segment et projetable dans son intégralité (devant le plan xy) et exprimé dans le repère de la caméra
pub fn project_line(line: Line3f, perspective_center_distance: f32) -> Line2f {
    Line2f::new(project_point(line.p0, perspective_center_distance), project_point(line.p1, perspective_center_distance))
}

fn project_point(point: Point3f, f: f32) -> Point2f {
    let x_proj = f * point.p.x / point.p.z;
    let y_proj = f * point.p.y / point.p.z;
    Point2f::new((x_proj, y_proj), point.color)
}

pub fn filter_primitive_3d(primitive: Primitive3f) -> Option<Primitive3f> {
    match primitive {
        Primitive3f::Line(line) => filter_line_3d(line).map(Primitive3f::Line),
        Primitive3f::Point(point) => if point.p.z > EPSILON { Some(Primitive3f::Point(point)) } else { None }
    }
}

//pour supprimer les segments situés derrière le plan de la camera (si un sengement coupe le plan, l'adapte pour qu'il soit projetable)
pub fn filter_line_3d(line: Line3f) -> Option<Line3f> {
    match (line.p0.p.z > 0.0, line.p1.p.z > 0.0) {
        (true, true) => Some(line),
        (true, false) => Some(adjusted_line_3d(line)),
        (false, true) => Some(adjusted_line_3d(line.inverted())),
        (false, false) => None,
    }
}

//pour un segment donc p0 est situé devant le plan xy (z >= 0) et p1 derriere, tranlate p1 sur le segment de manière à le positionner sur xy
fn adjusted_line_3d(mut line: Line3f) -> Line3f {
    let dxdz = (line.p1.p.x - line.p0.p.x) / (line.p1.p.z - line.p0.p.z);
    let dydz = (line.p1.p.y - line.p0.p.y) / (line.p1.p.z - line.p0.p.z);

    let dx_intersection = dxdz * (-line.p0.p.z + EPSILON);
    let dy_intersection = dydz * (-line.p0.p.z + EPSILON);

    let p1 = Vector3::new(line.p0.p.x + dx_intersection, line.p0.p.y + dy_intersection, EPSILON);
    line.p1.p = p1;

    line
}