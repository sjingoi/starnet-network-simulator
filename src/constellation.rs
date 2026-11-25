use std::f64::consts::{PI, TAU};

use glam::{DAffine3, DVec3};
use kiss3d::{color::YELLOW, renderer::Polyline3d};

use crate::{
    orbits::{Epoch, Orbit, Planet},
    topology::Topology,
    utils::VecDVec3Ext,
};

const ORBIT_ALT_METRES: f64 = 400_000.0;
const ORBIT_INCL_RAD: f64 = 53.25 * PI / 180.0;
const ORBIT_SAMPLES: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SatelliteId {
    pub group: u8,
    pub index: u8,
}

#[derive(Debug)]
pub struct Sattelite {
    pub id: SatelliteId,
    pub epoch: Epoch,
}

impl Sattelite {
    pub fn position(&self, orbit: &Orbit<'_>, time_sec: f64) -> DVec3 {
        let sec_per_orbit = orbit.period();
        let time_since_epoch_sec = time_sec - self.epoch.time_0;
        let orbits = time_since_epoch_sec / sec_per_orbit;
        let dist_since_epoch_rad = orbits * TAU;
        let dist_since_t0_rad = dist_since_epoch_rad + self.epoch.mean_anomaly;
        let mean_anomaly_pos = dist_since_t0_rad % TAU;
        orbit.params.position(mean_anomaly_pos)
    }
}

#[derive(Debug)]
pub struct SatelliteGroup<'a> {
    pub orbit: Orbit<'a>,
    pub satellites: Vec<Sattelite>,
}

#[derive(Debug)]
pub struct Constellation<'a> {
    pub groups: Vec<SatelliteGroup<'a>>,
}

impl<'a> Constellation<'a> {
    pub fn new(planet: &'a Planet, groups: u8, sats_per_group: u8) -> Self {
        let groups = (0..groups)
            .map(|group| {
                let orbit = Orbit::new(
                    planet,
                    ORBIT_ALT_METRES,
                    ORBIT_ALT_METRES,
                    ORBIT_INCL_RAD,
                    TAU * group as f64 / groups as f64,
                    0.0,
                );
                let satellites = (0..sats_per_group)
                    .map(|i| Sattelite {
                        id: SatelliteId { group, index: i },
                        epoch: Epoch::new(
                            i as f64 / sats_per_group as f64,
                            orbit.period() * TAU * group as f64 / (sats_per_group as f64).powi(2),
                        ),
                    })
                    .collect();
                SatelliteGroup { orbit, satellites }
            })
            .collect();

        Self { groups }
    }

    pub fn render_ellipses(&self, scale: f64) -> Vec<Polyline3d> {
        self.groups
            .iter()
            .map(|group| {
                let mut verts = group
                    .orbit
                    .params
                    .generate_points(ORBIT_SAMPLES)
                    .transform(DAffine3::from_scale(DVec3::splat(1.0 / scale)))
                    .to_f32();
                verts.push(verts[0]);

                Polyline3d::new(verts)
                    .with_color(YELLOW)
                    .with_width(0.5)
                    .with_perspective(false)
            })
            .collect()
    }

    pub fn topology(&self, time: f64, min_range: f64, max_range: f64) -> Topology {
        Topology::new(self, time, min_range, max_range)
    }
}
