use std::f64::consts::TAU;

use glam::DVec3;

use crate::{
    orbits::{Epoch, Orbit},
    topology::Topology,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SatelliteId {
    pub group: u8,
    pub index: u8,
}

#[derive(Debug)]
pub struct Sattelite<'a> {
    pub id: SatelliteId,
    pub orbit: &'a Orbit<'a>,
    pub epoch: Epoch,
}

impl Sattelite<'_> {
    pub fn position(&self, time_sec: f64) -> DVec3 {
        let sec_per_orbit = self.orbit.period();
        let time_since_epoch_sec = time_sec - self.epoch.time_0;
        let orbits = time_since_epoch_sec / sec_per_orbit;
        let dist_since_epoch_rad = orbits * TAU;
        let dist_since_t0_rad = dist_since_epoch_rad + self.epoch.mean_anomaly;
        let mean_anomaly_pos = dist_since_t0_rad % TAU;
        self.orbit.params.position(mean_anomaly_pos)
    }
}

#[derive(Debug)]
pub struct SatelliteGroup<'a> {
    pub satellites: Vec<Sattelite<'a>>,
}

#[derive(Debug)]
pub struct Constellation<'a> {
    pub groups: Vec<SatelliteGroup<'a>>,
}

impl Constellation<'_> {
    pub fn topology(&self, time: f64, min_range: f64, max_range: f64) -> Topology {
        Topology::new(self, time, min_range, max_range)
    }
}
