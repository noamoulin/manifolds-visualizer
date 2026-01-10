use std::f32::consts::{FRAC_PI_2, PI};
use derive_more::Constructor;
use itertools::Itertools;
use nalgebra::Vector3;

pub struct Surface {
    parts: Vec<SurfaceParam>,
    u_points_number: u32,
    v_points_number: u32,
}

impl Surface {
    pub fn new_sphere(r: f32, u_points_number: u32, v_points_number: u32) -> Self {
        let parts = vec![SurfaceParam::new_sphere(r)];

        Self {parts, u_points_number, v_points_number}
    }

    pub fn new_torus(r: f32, r_rev: f32, u_points_number: u32, v_points_number: u32) -> Self {
        let parts = vec![SurfaceParam::new_torus(r, r_rev)];

        Self {parts, u_points_number, v_points_number}
    }

    pub fn new_mobius(r: f32, w: f32, u_points_number: u32, v_points_number: u32) -> Self {
        let parts = vec![SurfaceParam::new_mobius(r, w)];

        Self {parts, u_points_number, v_points_number}
    }

    pub fn new_cube_sphere(r: f32, n: u32) -> Self {
        let parts = (0..6)
            .map(|i| SurfaceParam::cube_sphere_face(r, i))
            .collect();

        Self {
            parts,
            u_points_number: n,
            v_points_number: n,
        }
    }

    pub fn new_boy(scale: f32, u: u32, v: u32) -> Self {
        let parts = vec![
            SurfaceParam::new_boy(scale),
        ];

        Self {
            parts,
            u_points_number: u,
            v_points_number: v,
        }
    }

    pub fn isos_u(&self) -> impl Iterator<Item = (Vector3<f32>, Vector3<f32>)> {
        self.parts.iter().flat_map(move |p| p.isos_u(self.u_points_number, self.v_points_number))
    }

    pub fn isos_v(&self) -> impl Iterator<Item = (Vector3<f32>, Vector3<f32>)> {
        self.parts.iter().flat_map(move |p| p.isos_v(self.u_points_number, self.v_points_number))
    }

    pub fn isolines(
        &self,
    ) -> impl Iterator<Item = (Vector3<f32>, Vector3<f32>)> {
        self.parts.iter().flat_map(move |p| p.isos_u(self.u_points_number, self.v_points_number).chain(p.isos_v(self.v_points_number, self.u_points_number)))
    }
}

#[derive(Constructor)]
pub struct SurfaceParam {
    f: Box<dyn Fn(f32, f32) -> Vector3<f32>>,
    u_range: (f32, f32),
    v_range: (f32, f32),
}

impl SurfaceParam {
    //sphere de rayon r
    pub fn new_sphere(r: f32) -> Self {
        let f = move |u: f32, v: f32| {
            let cos_u = u.cos();
            let sin_u = u.sin();
            let cos_v = v.cos();
            let sin_v = v.sin();

            let x = r * cos_u * cos_v;
            let y = r * sin_u * cos_v;
            let z = r * sin_v;

            Vector3::new(x, y, z)
        };

        SurfaceParam::new(Box::new(f), (-PI, PI), (-FRAC_PI_2, FRAC_PI_2))
    }

    pub fn new_torus(r: f32, r_rev: f32) -> Self {
        let f = move |u: f32, v: f32| {
            let cos_u = u.cos();
            let sin_u = u.sin();
            let cos_v = v.cos();
            let sin_v = v.sin();
    
            let x = (r + r_rev * cos_v) * cos_u;
            let y = (r + r_rev * cos_v) * sin_u;
            let z = r_rev * sin_v;
    
            Vector3::new(x, y, z)
        };
    
        SurfaceParam::new(Box::new(f), (0.0, 2.0 * PI), (0.0, 2.0 * PI))
    }

    pub fn new_mobius(r: f32, w: f32) -> Self {
        let f = move |u: f32, v: f32| {
            let cos_u = u.cos();
            let sin_u = u.sin();
            let cos_u2 = (u * 0.5).cos();
            let sin_u2 = (u * 0.5).sin();

            let x = (r + v * cos_u2) * cos_u;
            let y = (r + v * cos_u2) * sin_u;
            let z = v * sin_u2;

            Vector3::new(x, y, z)
        };

        SurfaceParam::new(
            Box::new(f),
            (0.0, 2.0 * PI),
            (-w, w),
        )
    }

    pub fn cube_sphere_face(r: f32, face: usize) -> Self {
        let f = move |u: f32, v: f32| {
            let x = u;
            let y = v;

            let p = match face {
                0 => Vector3::new( 1.0,  x,  y),
                1 => Vector3::new(-1.0,  x,  y),
                2 => Vector3::new( x,  1.0,  y),
                3 => Vector3::new( x, -1.0,  y),
                4 => Vector3::new( x,  y,  1.0),
                _ => Vector3::new( x,  y, -1.0),
            };

            let p = p.normalize();
            r * p
        };

        SurfaceParam::new(Box::new(f), (-1.0, 1.0), (-1.0, 1.0))
    }

    pub fn new_boy(scale: f32) -> Self {
        let f = move |u: f32, v: f32| {
            let su = u.sin();
            let cu = u.cos();
            let s2u = (2.0 * u).sin();
            let c2u = (2.0 * u).cos();

            let cv = v.cos();
            let sv = v.sin();
            let c2v = (2.0 * v).cos();
            let s2v = (2.0 * v).sin();

            let x = 0.5 * (su * cv + s2u * c2v);
            let y = 0.5 * (su * sv - s2u * s2v);
            let z = 0.5 * (cu - c2u);

            scale * Vector3::new(x, y, z)
        };

        SurfaceParam::new(
            Box::new(f),
            (0.0, PI),
            (0.0, 2.0 * PI),
        )
    }
    
    //segments dans tout l'intervalle sous-echantilloné des u pour chaque valeurs du sous echantillonage des v
    pub fn isos_v(
        &self,
        v_points_number: u32,
        u_points_number: u32,
    ) -> impl Iterator<Item = (Vector3<f32>, Vector3<f32>)> {
        let v_values = regular_sample(self.v_range.0, self.v_range.1, v_points_number);
        let u_values: Vec<f32> =
            regular_sample(self.u_range.0, self.u_range.1, u_points_number).collect();
        v_values.flat_map(move |v| {
            u_values
                .clone()
                .into_iter()
                .map(move |u| (self.f)(u, v))
                .tuple_windows()
        })
    }

    //segments dans tout l'intervalle sous-echantilloné des v pour chaque valeurs du sous echantillonage des u
    pub fn isos_u(
        &self,
        u_points_number: u32,
        v_points_number: u32,
    ) -> impl Iterator<Item = (Vector3<f32>, Vector3<f32>)> {
        let u_values = regular_sample(self.u_range.0, self.u_range.1, u_points_number);
        let v_values: Vec<f32> =
            regular_sample(self.v_range.0, self.v_range.1, v_points_number).collect();
        u_values.flat_map(move |u| {
            v_values
                .clone()
                .into_iter()
                .map(move |v| (self.f)(u, v))
                .tuple_windows()
        })
    }

    //isos_u U isos_v -> the wireframe
    pub fn isolines(
        &self,
        u_points_number: u32,
        v_points_number: u32,
    ) -> impl Iterator<Item = (Vector3<f32>, Vector3<f32>)> {
        self.isos_u(u_points_number, v_points_number)
            .chain(self.isos_v(v_points_number, u_points_number))
    }
}

//sous echantillonage regulier de [a; b] en n points
fn regular_sample(a: f32, b: f32, n: u32) -> impl Iterator<Item = f32> {
    (0..n).map(move |i| a + (b - a) * i as f32 / (n - 1) as f32)
}
