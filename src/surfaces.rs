use std::f32::consts::{FRAC_PI_2, PI};

use derive_more::Constructor;
use itertools::Itertools;
use nalgebra::Vector3;

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

    pub fn new_torus(R: f32, r: f32) -> Self {
        let f = move |u: f32, v: f32| {
            let cos_u = u.cos();
            let sin_u = u.sin();
            let cos_v = v.cos();
            let sin_v = v.sin();
    
            let x = (R + r * cos_v) * cos_u;
            let y = (R + r * cos_v) * sin_u;
            let z = r * sin_v;
    
            Vector3::new(x, y, z)
        };
    
        SurfaceParam::new(Box::new(f), (0.0, 2.0 * PI), (0.0, 2.0 * PI))
    }

    pub fn new_mobius(R: f32, w: f32) -> Self {
        let f = move |u: f32, v: f32| {
            let cos_u = u.cos();
            let sin_u = u.sin();
            let cos_u2 = (u * 0.5).cos();
            let sin_u2 = (u * 0.5).sin();

            let x = (R + v * cos_u2) * cos_u;
            let y = (R + v * cos_u2) * sin_u;
            let z = v * sin_u2;

            Vector3::new(x, y, z)
        };

        SurfaceParam::new(
            Box::new(f),
            (0.0, 2.0 * PI),
            (-w, w),
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
    pub fn isovertices(
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
