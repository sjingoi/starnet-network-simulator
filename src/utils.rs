use std::f64::consts::TAU;

use glam::{DAffine2, DAffine3, DVec2, DVec3};
use kiss3d::glamx::Vec3;

pub trait DVec2Ext {
    fn to_3d(self) -> DVec3;
}

impl DVec2Ext for DVec2 {
    fn to_3d(self) -> DVec3 {
        DVec3::new(self.x, self.y, 0.0)
    }
}

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

#[derive(Debug)]
pub struct Ray2D {
    p0: DVec2,
    d: DVec2,
}

impl Ray2D {
    pub fn new(point: DVec2, direction: DVec2) -> Self {
        Ray2D { p0: point, d: direction }
    }

    pub fn transform(&self, affine: DAffine2) -> Self {
        Ray2D {
            p0: affine.transform_point2(self.p0),
            d: affine.transform_vector2(self.d)
        }
    }

    pub fn at(&self, lambda: f64) -> DVec2 {
        self.p0 + lambda * self.d
    }

    pub fn ucirc_intersect(&self) -> Option<f64> {
        let a = self.d.dot(self.d);
        let b = 2.0 * self.p0.dot(self.d);
        let c = self.p0.dot(self.p0) - 1.0;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 || a == 0.0 {
            return None;
        }

        let sq = disc.sqrt();
        let t1 = (-b - sq) / (2.0 * a); // smaller root
        let t2 = (-b + sq) / (2.0 * a); // larger root

        let t = if t1 >= 0.0 {
            t1
        } else if t2 >= 0.0 {
            t2
        } else {
            return None;
        };

        Some(t)
    }
}