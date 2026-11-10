use std::{f64::consts::PI, f64::consts::TAU};
use glam::{DAffine3, DVec3};

trait FloatExt {
    fn ecc_anomaly(self, orbit: &OrbitParams) -> Self;
    fn true_anomoly(self, orbit: &OrbitParams) -> Self;
}

impl FloatExt for f64 {
    fn ecc_anomaly(self, orbit: &OrbitParams) -> f64 {
        let m: f64 = self.rem_euclid(TAU);
        let e: f64 = orbit.eccentricity;
    
        // Circular orbit: E = M exactly.
        if e == 0.0 {
            return m;
        }
    
        let mut ecc = m + 0.85 * e * if m < PI { 1.0 } else { -1.0 };
    
        // Newton's method
        const TOL: f64 = 1e-14;
        const MAX_ITER: usize = 50;
    
        for _ in 0..MAX_ITER {
            let (s, c) = ecc.sin_cos();
            let f = ecc - e * s - m;
            let f_prime = 1.0 - e * c;
            let delta = f / f_prime;
            ecc -= delta;
            if delta.abs() < TOL {
                break;
            }
        }
    
        ecc.rem_euclid(TAU)
    }

    fn true_anomoly(self, orbit: &OrbitParams) -> f64 {
        let m: f64 = self.rem_euclid(TAU);
        let e: f64 = orbit.eccentricity;
        let (s, c) = (m / 2.0).sin_cos();

        2.0 * ((1.0 + e).sqrt() * s).atan2((1.0 - e).sqrt() * c)   
    }
}

trait VecDVec3Ext {
    fn transform(self, transform: DAffine3) -> Self;
}

impl VecDVec3Ext for Vec<DVec3> {
    fn transform(mut self, affine: DAffine3) -> Self {
        for p in &mut self {
            *p = affine.transform_point3(*p);
        }

        self
    }
}

fn sample_u_circ(n: u32) -> Vec<DVec3> {
    (0..n).map(|i| -> DVec3 {
            let angle = i as f64 * TAU / n as f64;
            let (s, c) = angle.sin_cos();
            DVec3::new(s, c, 0.0)
    }).collect()
}

#[derive(Debug)]
struct OrbitParams {
    semi_maj_axis: f64, // Half of the elipse's major axis diameter
    eccentricity: f64, // 0 - Circular orbit, closer to 1 - stretched elipse
    inclincation: f64, // Inclidation relative to parent equator
    asc_node_lon: f64, // Longitue of the asending node (latitude where orbit crosses equator going up)
    arg_periapsis: f64, // Angle betwee asc_node_lon and periapsis along orbital plane
}

impl OrbitParams {
    fn semi_min_axis(&self) -> f64 {
        self.semi_maj_axis * (1.0 - self.eccentricity.powi(2)).sqrt()
    }

    fn affine_from_ucirc(&self) -> DAffine3 {
        // let peri = self.semi_maj_axis * (1.0 - self.eccentricity);
        let scale_t = DAffine3::from_scale(glam::DVec3 {
            x: self.semi_maj_axis,
            y: self.semi_min_axis(),
            z: 0.0
        });
        let trans_t = DAffine3::from_translation(glam::DVec3 {
            x: self.semi_maj_axis * self.eccentricity,
            y: 0.0,
            z: 0.0
        });
        let rot_arg_peri = DAffine3::from_rotation_z(self.arg_periapsis);
        let rot_incl = DAffine3::from_rotation_x(self.inclincation);
        let rot_asc_node = DAffine3::from_rotation_z(self.asc_node_lon);

        rot_asc_node * rot_incl * rot_arg_peri * trans_t * scale_t
    }

    fn generate_points(&self, n: u32) -> Vec<DVec3> {
        let affine = self.affine_from_ucirc();
        sample_u_circ(n).transform(affine)
    }
}

#[derive(Debug)]
struct Planet {
    mass: f64,
    radius: f64,
}

#[derive(Debug)]
struct Orbit<'a> {
    parent: &'a Planet,
    params: OrbitParams,
}

impl<'a> Orbit<'a> {
    fn new(parent: &'a Planet, pe_alt: f64, ap_alt: f64, inclination: f64, asc_node_lon: f64, arg_periapsis: f64) -> Orbit<'a> {
        let periapsis = pe_alt + parent.radius;
        let apoapsis = ap_alt + parent.radius;
        let semi_maj_axis = (periapsis + apoapsis) / 2.0;
        let eccentricity = 1.0 - periapsis / semi_maj_axis;

        Orbit { 
            parent: parent,
            params: OrbitParams {
                semi_maj_axis: semi_maj_axis,
                eccentricity: eccentricity,
                inclincation: inclination,
                asc_node_lon: asc_node_lon,
                arg_periapsis: arg_periapsis
            },
        }
    }
}

#[derive(Debug)]
struct Epoch {
    mean_anomaly: f64, // Position in orbit
    time_0: f64, // Time at which the object is at that position
}

#[derive(Debug)]
struct Sattelite<'a> {
    orbit: Orbit<'a>,
    epoch: Epoch
}

fn main() {

    let earth = Planet {
        mass: 5.9722e24,
        radius: 6.371e6,
        // radius: 1.0
    };

    let group1_orbit = Orbit::new(
        &earth, 
        200000.0, // 200km
        8000000.0, // 8000km
        PI/2.0, 
        0.0,
        0.0,
    );

    let sat1 = Sattelite {
        orbit: group1_orbit,
        epoch: Epoch { mean_anomaly: 0.0, time_0: 0.0 }
    };

    let points = sat1.orbit.params.generate_points(100);
    println!("{:#?}", points);
    // println!("{:#?}", sat1);
    // println!("{:#?}", sat1.orbit.params.affine_from_ucirc());
    
}
