use derive_more::Constructor;
use nalgebra::{Matrix4, Vector3};

use crate::objects::{pointclouds::PointCloud, surfaces::Surface};

pub mod surfaces;
pub mod pointclouds;

const DEFAULT_LINE_COLOR: u32 = 0xffffff;

#[derive(Constructor)]
pub struct Object {
    geometry: Geometry,
    transform: Matrix4<f32>,
    color: u32,
}

impl Object {
    pub fn primitives(&self) -> impl Iterator<Item = Primitive3f> {
        self.geometry.primitives().map(|elm| elm.with_color(self.color))
    }
}

pub enum Geometry {
    Surface(Surface),
    PointCloud(PointCloud),
}

impl Geometry {
    pub fn primitives(&self) -> impl Iterator<Item = Primitive3f> {
        match self {
            Geometry::Surface(surface) => surface.isolines().map(Primitive3f::from),
            Geometry::PointCloud(cloud) => todo!()
        }
    }
}

impl From<(Vector3<f32>, Vector3<f32>)> for Primitive3f {
    fn from(value: (Vector3<f32>, Vector3<f32>)) -> Self {
        Self::Line(Line3f::new(Point3f::new(value.0, DEFAULT_LINE_COLOR), Point3f::new(value.1, DEFAULT_LINE_COLOR))) //blanc par defaut
    }
}

pub enum Primitive3f {
    Line(Line3f),
    Point(Point3f),
}

impl Primitive3f {
    fn with_color(mut self, color: u32) -> Self{
        match &mut self {
            Primitive3f::Line(line) => {
                line.p0.color = color;
                line.p1.color = color;
            }
            Primitive3f::Point(point) => point.color = color,
        }

        self
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

#[derive(Constructor)]
pub struct Point3f {
    pub p: Vector3<f32>,
    pub color: u32,
}