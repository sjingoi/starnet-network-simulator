mod orbits;
mod utils;
use orbits::{Planet, Orbit, Sattelite, Epoch};
use std::{f64::consts::PI};

fn main() {

    let earth = Planet {
        mass: 5.9722e24,
        radius: 6.371e6,
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
        epoch: Epoch::new(0.0, 0.0)
    };

    let points = sat1.orbit.params.generate_points(100);
    println!("{:#?}", points);
    // println!("{:#?}", sat1);
    // println!("{:#?}", sat1.orbit.params.affine_from_ucirc());
    
}
