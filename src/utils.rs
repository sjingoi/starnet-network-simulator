use std::f64::consts::TAU;

use glam::{DAffine3, DVec3};
use kiss3d::glamx::Vec3;

pub trait VecDVec3Ext {
    fn transform(self, transform: DAffine3) -> Self;
    fn to_f32(self) -> Vec<Vec3>;
}

impl VecDVec3Ext for Vec<DVec3> {
    fn transform(mut self, affine: DAffine3) -> Self {
        for p in &mut self {
            *p = affine.transform_point3(*p);
        }

        self
    }

    fn to_f32(self) -> Vec<Vec3> {
        self.iter()
            .map(|p| Vec3::new(p.x as f32, p.y as f32, p.z as f32))
            .collect()
    }
}

pub fn sample_u_circ(n: u32) -> Vec<DVec3> {
    (0..n).map(|i| -> DVec3 {
            let angle = i as f64 * TAU / n as f64;
            let (s, c) = angle.sin_cos();
            DVec3::new(s, c, 0.0)
    }).collect()
}