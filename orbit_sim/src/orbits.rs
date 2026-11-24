use std::{cell::OnceCell, f64::consts::{PI, TAU}};
use crate::utils::{VecDVec3Ext, sample_u_circ};
use glam::{DAffine3, DVec3};

const G: f64 = 6.67430e-11;

trait OrbitalFloatsExt {
    fn ecc_anomaly(self, orbit: &OrbitParams) -> Self;
    fn true_anomoly(self, orbit: &OrbitParams) -> Self;
}

impl OrbitalFloatsExt for f64 {
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

#[derive(Debug)]
pub struct OrbitParams {
    // 2d orbit params that define the orbit's shape on it's plane
    semi_maj_axis: f64, // Half of the elipse's major axis diameter
    eccentricity: f64, // 0 - Circular orbit, closer to 1 - stretched elipse
    arg_periapsis: f64, // Angle betwee asc_node_lon and periapsis along orbital plane
    
    // 3d orbit params that define how the orbit plane (2d orbit) is oriented in 3d space
    inclincation: f64, // Inclidation relative to parent equator
    asc_node_lon: f64, // Longitue of the asending node (latitude where orbit crosses equator going up)

    // Memoized calculations
    affine_3d: OnceCell<DAffine3>,
    affine_2d_to_3d: OnceCell<DAffine3>,
}

impl OrbitParams {
    fn semi_min_axis(&self) -> f64 {
        self.semi_maj_axis * (1.0 - self.eccentricity.powi(2)).sqrt()
    }

    fn affine_from_ucirc(&self) -> DAffine3 {
        *self.affine_3d.get_or_init(|| {
            // let peri = self.semi_maj_axis * (1.0 - self.eccentricity);
            let scale_t = DAffine3::from_scale(glam::DVec3 {
                x: self.semi_maj_axis,
                y: self.semi_min_axis(),
                z: 0.0
            });
            let trans_t = DAffine3::from_translation(glam::DVec3 {
                x: -1.0 * self.semi_maj_axis * self.eccentricity,
                y: 0.0,
                z: 0.0
            });
            let rot_arg_peri = DAffine3::from_rotation_z(self.arg_periapsis);
            let rot_incl = DAffine3::from_rotation_x(self.inclincation);
            let rot_asc_node = DAffine3::from_rotation_z(self.asc_node_lon);
    
            rot_asc_node * rot_incl * rot_arg_peri * trans_t * scale_t
        })
    }

    fn affine_2d_to_3d(&self) -> DAffine3 {
        *self.affine_2d_to_3d.get_or_init(|| {
            let rot_incl = DAffine3::from_rotation_x(self.inclincation);
            let rot_asc_node = DAffine3::from_rotation_z(self.asc_node_lon);

            rot_asc_node * rot_incl
        })
    }

    pub fn generate_points(&self, n: u32) -> Vec<DVec3> {
        let affine = self.affine_from_ucirc();
        sample_u_circ(n).transform(affine)
    }

    pub fn position(&self, mean_anomaly: f64) -> DVec3 {
        let ecc = self.eccentricity;
        let true_anomaly = mean_anomaly.ecc_anomaly(self).true_anomoly(self);
        let (s, c) = true_anomaly.sin_cos();
        let radius = self.semi_maj_axis * (1.0 - ecc.powi(2)) / (1.0 + ecc * c);

        let point_relative = DVec3::new(
            radius * c,
            radius * s,
            0.0
        );

        let point_plane = DAffine3::from_rotation_z(self.arg_periapsis)
            .transform_point3(point_relative);
        
        self.affine_2d_to_3d().transform_point3(point_plane)
    }
}

#[derive(Debug)]
pub struct Planet {
    pub mass: f64, // Mass in kg
    pub radius: f64, // Radius in metres
    pub sr_day: f64, // Sidereal day length in seconds
}

#[derive(Debug)]
pub struct Orbit<'a> {
    pub parent: &'a Planet,
    pub params: OrbitParams,
}

impl<'a> Orbit<'a> {
    pub fn new(parent: &'a Planet, pe_alt: f64, ap_alt: f64, inclination: f64, asc_node_lon: f64, arg_periapsis: f64) -> Orbit<'a> {
        let periapsis = pe_alt + parent.radius;
        let apoapsis = ap_alt + parent.radius;
        let semi_maj_axis = (periapsis + apoapsis) / 2.0;
        let eccentricity = 1.0 - periapsis / semi_maj_axis;

        Orbit { 
            parent: parent,
            params: OrbitParams {
                semi_maj_axis: semi_maj_axis,
                eccentricity: eccentricity,
                arg_periapsis: arg_periapsis,

                inclincation: inclination,
                asc_node_lon: asc_node_lon,

                affine_3d: OnceCell::new(), 
                affine_2d_to_3d: OnceCell::new()
            },
        }
    }

    pub fn period(&self) -> f64 {
        TAU * (self.params.semi_maj_axis.powi(3) / (G * self.parent.mass)).sqrt()
    }
}

#[derive(Debug)]
pub struct Epoch {
    pub(crate) mean_anomaly: f64, // Position in orbit
    pub(crate) time_0: f64, // Time at which the object is at that position
}

impl Epoch {
    pub fn new(position: f64, reference_time: f64) -> Epoch {
        Epoch {
            mean_anomaly: position * TAU,
            time_0: reference_time
        }
    }
}
