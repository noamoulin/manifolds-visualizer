use derive_more::Constructor;
use nalgebra::{Matrix4, Vector3};

use crate::objects::{pointclouds::PointCloud, surfaces::Surface};

pub mod surfaces;
pub mod pointclouds;

const DEFAULT_LINE_COLOR: u32 = 0xffffff;

#[derive(Constructor)]
pub struct Object {
    geometry: Geometry,
    pub local_to_world: Matrix4<f32>,
    color: u32,
}

impl Object {
    pub fn raw_primitives(&self) -> impl Iterator<Item = Primitive3f> {
        self.geometry.primitives().map(|elm| elm.with_color(self.color))
    }

    pub fn primitives(&self) -> impl Iterator<Item = Primitive3f> {
        self.raw_primitives().map(|p| p.transformed(self.local_to_world))
    }

    pub fn rotate_x(&mut self, d_angle: f32) {
        let cos = d_angle.cos();
        let sin = d_angle.sin();

        let rotation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, cos, -sin,0.0,
            0.0, sin, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        self.local_to_world = self.local_to_world * rotation_matrix;
    }

    pub fn rotate_y(&mut self, d_angle: f32) {
        let cos = d_angle.cos();
        let sin = d_angle.sin();

        let rotation_matrix = Matrix4::new(
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0,  0.0,
            sin, 0.0, cos,  0.0,
            0.0, 0.0, 0.0,  1.0
        );

        self.local_to_world = self.local_to_world * rotation_matrix;
    }

    pub fn rotate_z(&mut self, d_angle: f32) {
        let cos = d_angle.cos();
        let sin = d_angle.sin();

        let rotation_matrix = Matrix4::new(
            cos, -sin, 0.0, 0.0,
            sin, cos,  0.0, 0.0,
            0.0, 0.0,  1.0, 0.0,
            0.0, 0.0,  0.0, 1.0,
         );

         self.local_to_world = self.local_to_world * rotation_matrix;
    }

    pub fn translate_relative(&mut self, dp: Vector3<f32>) {
        let translation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, dp.x,
            0.0, 1.0, 0.0, dp.y,
            0.0, 0.0, 1.0, dp.z,
            0.0, 0.0, 0.0, 1.0
        );

        self.local_to_world = self.local_to_world * translation_matrix;
    }

    pub fn translate_absolute(&mut self, dp: Vector3<f32>) {
        let translation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, dp.x,
            0.0, 1.0, 0.0, dp.y,
            0.0, 0.0, 1.0, dp.z,
            0.0, 0.0, 0.0, 1.0
        );

        self.local_to_world = translation_matrix * self.local_to_world;
    }
}

pub enum Geometry {
    Surface(Surface),
    PointCloud(PointCloud),
}

impl Geometry {
    pub fn primitives(&self) -> impl Iterator<Item = Primitive3f> + '_ {
        match self {
            Geometry::Surface(surface) => either::Left(surface.isolines().map(Primitive3f::from)),
            Geometry::PointCloud(cloud) => either::Right(cloud.points().map(Primitive3f::from)),
        }
    }
}

impl From<(Vector3<f32>, Vector3<f32>)> for Primitive3f {
    fn from(value: (Vector3<f32>, Vector3<f32>)) -> Self {
        Self::Line(Line3f::new(Point3f::new(value.0, DEFAULT_LINE_COLOR), Point3f::new(value.1, DEFAULT_LINE_COLOR))) //blanc par defaut
    }
}

impl From<Vector3<f32>> for Primitive3f {
    fn from(value: Vector3<f32>) -> Self {
        Self::Point(Point3f::new(value, DEFAULT_LINE_COLOR)) //blanc par defaut
    }
}

impl From<Point3f> for Primitive3f {
    fn from(value: Point3f) -> Self {
        Self::Point(value)
    }
}

pub enum Primitive3f {
    Line(Line3f),
    Point(Point3f),
}

impl Primitive3f {
    //ajoute de la coouleur si il n'y en a pas
    fn with_color(self, color: u32) -> Self  {
        match &self {
            Self::Line(line) => {
                if line.p0.color > 0 {
                    self
                }
                else {
                    let p0 = Point3f::new(line.p0.p, color);
                    let p1 = Point3f::new(line.p1.p, color);
                    Self::Line(Line3f::new(p0, p1))
                }
            }
            Self::Point(point) => {
                if point.color > 0 {
                    self
                }
                else {
                    Self::Point(Point3f::new(point.p, color))
                }
            },
        }
    }

    fn transformed(self, transform: Matrix4<f32>) -> Self {
        match self {
            Primitive3f::Point(point) => Primitive3f::Point(point.transformed(transform)),

            Primitive3f::Line(line) => {
                let p0 = line.p0.transformed(transform);
                let p1 = line.p1.transformed(transform);

                Primitive3f::Line(Line3f::new(p0, p1))
            }
        }
    }
}

#[derive(Constructor)]
pub struct Line3f {
    pub p0: Point3f,
    pub p1: Point3f,
}

impl Line3f {
    pub fn inverted(&self) -> Self {
        let p1 = self.p0.p;
        let p0 = self.p1.p;
        let color = self.p0.color;

        Self {
            p1: Point3f::new(p1, color),
            p0: Point3f::new(p0, color),
        }
    }
}

#[derive(Constructor, Clone)]
pub struct Point3f {
    pub p: Vector3<f32>,
    pub color: u32,
}

impl Point3f {
    pub fn transformed(self, transform: Matrix4<f32>) -> Self {
        let mut homogeneous = self.p.to_homogeneous();
        homogeneous.w = 1.0;

        let homogeneous = transform * homogeneous;
        Self::new(Vector3::<f32>::new(homogeneous.x, homogeneous.y, homogeneous.z), self.color)
    }
}