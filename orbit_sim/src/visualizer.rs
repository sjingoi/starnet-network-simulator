use kiss3d::{camera::OrbitCamera3d, color::{BLUE, GREEN, LIME_GREEN, RED, YELLOW}, glamx::{Quat, Vec3}, scene::SceneNode3d, window::Window};
use std::{f64::consts::{PI, TAU}, path::Path, time::{SystemTime}};

use crate::{constellation::Constellation, orbits::Planet};

const SCALE: f64 = 1e7;
const TIME_SCALE: f64 = 1.0;
const MAX_RANGE_METRES: f64 = 700_000.0;
const MIN_RANGE_METRES: f64 = 400_000.0;
const TOPOLOGY_INTERVAL_SEC: f64 = 10.0;

pub fn visualize() {
    kiss3d::pollster::block_on(visualize_async());
}

async fn visualize_async() {

    let earth = Planet {
        mass: 5.9722e24,
        radius: 6.371e6,
        sr_day: 8.6164091e4,
    };

    let constellation = Constellation::new(&earth, 32, 64);
    let ellipses = constellation.render_ellipses(SCALE);

    // Plot
    let mut window = Window::new("Orbit shape").await;
    window.set_ambient(1.0);
    // Start at an angle: with Z as the up axis, an eye directly on +Z would be
    // looking straight down the rotation axis (a degenerate view).
    
    let mut camera = OrbitCamera3d::new_with_frustum(PI as f32 / 4.0, 0.01, 1000.0, Vec3::new(20.0, -20.0, 15.0), Vec3::ZERO);
    // Make Z "up" so dragging left/right spins the view around the Z axis.
    camera.set_up_axis(Vec3::Z);
    let mut scene = SceneNode3d::empty();

    // kiss3d's sphere has its poles on Y, with the image's top row (north) at -Y.
    // Rotating -90° about X sends -Y to +Z, so north points up the Z axis.
    let earth_base_rot = Quat::from_axis_angle(Vec3::X, -std::f32::consts::FRAC_PI_2);
    let mut earth_sphere = scene.add_sphere((earth.radius / SCALE) as f32)
        .set_texture_from_file(Path::new("assets/EarthColorMap-4k.jpg"), "earth")
        .set_rotation(earth_base_rot);

    // Axis lines run from -AXIS_LEN to +AXIS_LEN; long enough to reach past the orbit.
    let axis_len: f32 = 150.0;
    let axes = [
        (Vec3::X, RED),
        (Vec3::Y, GREEN),
        (Vec3::Z, BLUE),
    ];

    let mut time_elapsed: f64 = 0.0;
    let mut ref_time = SystemTime::now();
    let mut topology = constellation.topology(time_elapsed, MIN_RANGE_METRES, MAX_RANGE_METRES);
    let mut last_topology = time_elapsed;
    println!("Satellites without neighbors: {}",
        topology.neighbors.values().filter(|neighbors| neighbors.is_empty()).count());
    println!("Average neighbors per satellite: {:.2}",
        topology.neighbors.values().map(Vec::len).sum::<usize>() as f64
            / topology.neighbors.len().max(1) as f64);

    while window.render_3d(&mut scene, &mut camera).await {
        // Lines only last one frame, so redraw every frame.
        for (dir, color) in axes {
            window.draw_line(-dir * axis_len, dir * axis_len, color, 0.5, false);
        }

        for ellipse in &ellipses {
            window.draw_polyline(ellipse);
        }

        for (start, end) in topology.links(&constellation, time_elapsed) {
            let start = start / SCALE;
            let end = end / SCALE;
            window.draw_line(
                Vec3::new(start.x as f32, start.y as f32, start.z as f32),
                Vec3::new(end.x as f32, end.y as f32, end.z as f32),
                LIME_GREEN,
                1.5,
                false,
            );
        }

        window.draw_point(Vec3 { x: (1.2e7 / SCALE) as f32, y: 0.0, z: 0.0 }, RED, 8.0);

        for group in &constellation.groups {
            for sat in &group.satellites {
                let sat_pos = sat.position(&group.orbit, time_elapsed) / SCALE;
                window.draw_point(Vec3::new(sat_pos.x as f32, sat_pos.y as f32, sat_pos.z as f32), YELLOW, 4.0);
            }
        }

        
        let time_now = SystemTime::now();
        let time_delta = time_now
        .duration_since(ref_time)
        .expect("Could not get time")
        .as_secs_f64();
    
        time_elapsed +=  TIME_SCALE * time_delta;
        ref_time = time_now;

        if time_elapsed - last_topology >= TOPOLOGY_INTERVAL_SEC {
            topology = constellation.topology(time_elapsed, MIN_RANGE_METRES, MAX_RANGE_METRES);
            last_topology = time_elapsed;
            println!("Satellites without neighbors: {}",
                topology.neighbors.values().filter(|neighbors| neighbors.is_empty()).count());
            println!("Average neighbors per satellite: {:.2}",
                topology.neighbors.values().map(Vec::len).sum::<usize>() as f64
                    / topology.neighbors.len().max(1) as f64);
        }

        // Spin about the world Z (polar) axis: one full turn per sidereal day.
        // Wrap in f64 before casting so the angle keeps f32 precision over long runs.
        let spin = (TAU * time_elapsed / earth.sr_day).rem_euclid(TAU) as f32;
        earth_sphere.set_rotation(Quat::from_axis_angle(Vec3::Z, spin) * earth_base_rot);
    }
    
}
