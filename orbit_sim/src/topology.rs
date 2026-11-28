use std::collections::HashMap;

use glam::DVec3;

use crate::constellation::{Address, Constellation};

pub struct Topology {
    pub neighbors: HashMap<Address, Vec<Address>>,
}

impl Topology {
    pub fn links(&self, constellation: &Constellation<'_>, time_sec: f64) -> Vec<(DVec3, DVec3)> {
        let positions: HashMap<_, _> = constellation
            .groups
            .iter()
            .flat_map(|group| {
                group.satellites.iter().map(move |satellite| {
                    (satellite.address, satellite.position(&group.orbit, time_sec))
                })
            })
            .collect();

        let mut lines = Vec::new();
        for (&from, neighbors) in &self.neighbors {
            for &to in neighbors {
                if from < to {
                    lines.push((positions[&from], positions[&to]));
                }
            }
        }
        lines
    }
}
