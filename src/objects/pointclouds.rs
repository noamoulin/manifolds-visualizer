use std::fs::File;
use std::path::Path;
use derive_more::Constructor;
use nalgebra::Vector3;
use ply_rs::parser::Parser;
use ply_rs::ply::{DefaultElement, Property};

#[derive(Constructor)]
pub struct PointCloud {
    points: Vec<Vector3<f32>>,
}

impl PointCloud {
    pub fn new_from_path(path: &str) -> anyhow::Result<Self> {
        load_point_cloud(path).map(Self::new)
    }

    pub fn points(&self) -> impl Iterator<Item = Vector3<f32>> {
        self.points.clone().into_iter() //il faut faire ça proprement ici, cloner le nuage n'est pas acceptable, mais bon pour faire simple au debut ça va
    }
}

//ply chargé en mémoire temporairement, il faudra utliser des arbres pour être serieux
pub fn load_point_cloud(path: &str) -> anyhow::Result<Vec<Vector3<f32>>> {
    let mut f = File::open(Path::new(path))?;
    let parser = Parser::<DefaultElement>::new();
    let ply = parser.read_ply(&mut f)?;

    let vertices = ply.payload.get("vertex")
        .ok_or_else(|| anyhow::anyhow!("Pas de vertex dans le PLY"))?;

    let mut points = Vec::with_capacity(vertices.len());

    for v in vertices {
        // helper pour convertir Property -> f32
        let get_f32 = |p: &Property| -> anyhow::Result<f32> {
            match p {
                Property::Float(f) => Ok(*f),
                Property::Double(d) => Ok(*d as f32),
                Property::Int(i) => Ok(*i as f32),
                Property::UInt(u) => Ok(*u as f32),
                _ => Err(anyhow::anyhow!("Type de propriété non supporté")),
            }
        };

        let x = v.get("x").ok_or_else(|| anyhow::anyhow!("Vertex sans x"))?;
        let y = v.get("y").ok_or_else(|| anyhow::anyhow!("Vertex sans y"))?;
        let z = v.get("z").ok_or_else(|| anyhow::anyhow!("Vertex sans z"))?;

        points.push(Vector3::new(get_f32(x)?, get_f32(y)?, get_f32(z)?));
    }

    Ok(points)
}

