use std::collections::HashMap;

use glam::DVec3;

use crate::constellation::{Constellation, SatelliteId};

pub struct Topology {
    pub neighbors: HashMap<SatelliteId, Vec<SatelliteId>>,
}

impl Topology {
    pub fn new(
        constellation: &Constellation<'_>,
        time: f64,
        min_range: f64,
        max_range: f64,
    ) -> Self {
        let satellites: Vec<(SatelliteId, DVec3)> = constellation
            .groups
            .iter()
            .flat_map(|group| &group.satellites)
            .map(|satellite| (satellite.id, satellite.position(time)))
            .collect();
        let mut neighbors: HashMap<SatelliteId, Vec<SatelliteId>> =
            satellites.iter().map(|(id, _)| (*id, Vec::new())).collect();

        let min_range_sqr = min_range.powi(2);
        let max_range_sqr = max_range.powi(2);

        for (i, &(id_a, position_a)) in satellites.iter().enumerate() {
            for &(id_b, position_b) in &satellites[i + 1..] {
                let dist_sqr = position_a.distance_squared(position_b);
                if min_range_sqr <= dist_sqr && dist_sqr <= max_range_sqr {
                    neighbors.get_mut(&id_a).unwrap().push(id_b);
                    neighbors.get_mut(&id_b).unwrap().push(id_a);
                }
            }
        }

        Self { neighbors }
    }

    pub fn links(&self, constellation: &Constellation<'_>, time_sec: f64) -> Vec<(DVec3, DVec3)> {
        let positions: HashMap<_, _> = constellation
            .groups
            .iter()
            .flat_map(|group| &group.satellites)
            .map(|satellite| (satellite.id, satellite.position(time_sec)))
            .collect();

        let mut lines = Vec::new();
        for (&from, neighbors) in &self.neighbors {
            for &to in neighbors {
                if (from.group, from.index) < (to.group, to.index) {
                    lines.push((positions[&from], positions[&to]));
                }
            }
        }
        lines
    }
}
