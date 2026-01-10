use std::fs::File;
use std::path::Path;

use anyhow::Result;
use derive_more::Constructor;
use nalgebra::Vector3;
use ply_rs::parser::Parser;
use ply_rs::ply::{DefaultElement, Property};
use linked_hash_map::LinkedHashMap;

use crate::objects::Point3f;

pub const DEFAULT_POINT_COLOR: u32 = 0x000000;

#[derive(Constructor)]
pub struct PointCloud {
    points: Vec<Point3f>,
}

impl PointCloud {
    pub fn new_from_path(path: &str) -> anyhow::Result<Self> {
        load_point_cloud(path).map(Self::new)
    }

    pub fn points(&self) -> impl Iterator<Item = Point3f> {
        self.points.clone().into_iter() //il faut faire ça proprement ici, cloner le nuage n'est pas acceptable, mais bon pour faire simple au debut ça va
    }
}

pub fn load_point_cloud(path: &str) -> Result<Vec<Point3f>> {
    let mut file = File::open(Path::new(path))?;
    let parser = Parser::<DefaultElement>::new();
    let ply = parser.read_ply(&mut file)?;

    let vertices = ply
        .payload
        .get("vertex")
        .ok_or_else(|| anyhow::anyhow!("Pas d'élément 'vertex' dans le PLY"))?;

    let mut points = Vec::with_capacity(vertices.len());

    for v in vertices {
        let x = get_f32(v, "x")?;
        let y = get_f32(v, "y")?;
        let z = get_f32(v, "z")?;

        let color = get_color(v).unwrap_or(DEFAULT_POINT_COLOR);

        points.push(Point3f::new(Vector3::new(x, y, z), color));
    }

    Ok(points)
}

fn get_f32(v: &LinkedHashMap<String, Property>, name: &str) -> Result<f32> {
    let p = v
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Vertex sans propriété '{}'", name))?;

    Ok(match p {
        Property::Float(f) => *f,
        Property::Double(d) => *d as f32,
        Property::Int(i) => *i as f32,
        Property::UInt(u) => *u as f32,
        Property::Short(i) => *i as f32,
        Property::UShort(u) => *u as f32,
        _ => return Err(anyhow::anyhow!("Type non supporté pour '{}'", name)),
    })
}

fn property_to_u8(p: &Property) -> Option<u8> {
    Some(match p {
        Property::UChar(v) => *v,
        Property::Char(v) => *v as u8,
        Property::UShort(v) => (*v).min(255) as u8,
        Property::Short(v) => (*v).clamp(0, 255) as u8,
        Property::UInt(v) => (*v).min(255) as u8,
        Property::Int(v) => (*v).clamp(0, 255) as u8,
        Property::Float(v) => (v.clamp(0.0, 1.0) * 255.0) as u8,
        Property::Double(v) => (v.clamp(0.0, 1.0) * 255.0) as u8,
        _ => return None,
    })
}

fn get_color(v: &LinkedHashMap<String, Property>) -> Option<u32> {
    let try_get = |names: &[&str]| {
        names.iter()
            .find_map(|&n| v.get(n))
            .and_then(property_to_u8)
    };

    let r = try_get(&["red", "r", "diffuse_red"])?;
    let g = try_get(&["green", "g", "diffuse_green"])?;
    let b = try_get(&["blue", "b", "diffuse_blue"])?;

    Some(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
}