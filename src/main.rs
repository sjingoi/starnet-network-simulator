mod orbits;
mod utils;
use kiss3d::{camera::OrbitCamera3d, color::{BLUE, GREEN, RED, YELLOW}, glamx::{Quat, Vec3}, renderer::Polyline3d, scene::SceneNode3d, window::Window};
use orbits::{Planet, Orbit};
use std::{f64::consts::{PI, TAU}, path::Path, time::{SystemTime}};
use glam::{DAffine3, DVec3};

use crate::{orbits::{Epoch, SatelliteId, Sattelite}, utils::VecDVec3Ext};

const SCALE: f64 = 1e7;
const TIME_SCALE: f64 = 1.0;

#[kiss3d::main]
async fn main() {

    let earth = Planet {
        mass: 5.9722e24,
        radius: 6.371e6,
        sr_day: 8.6164091e4,
    };

    const NUM_GROUPS: u32 = 32;
    const NUM_SATS_PER_GROUP: u32 = 64;

    let orbits: Vec<Orbit> = (0..NUM_GROUPS).map(|i| Orbit::new(
        &earth, 
        400000.0, // 400km
        400000.0, // 400km
        53.25 * PI/180.0, 
        TAU * i as f64 / NUM_GROUPS as f64,
        0.0,
    )).collect();

    let groups: Vec<Vec<Sattelite>> = orbits.iter().enumerate().map(|(group_i, orbit)| {
        (0..NUM_SATS_PER_GROUP).map(|i| -> Sattelite {
            Sattelite {
                id: SatelliteId {
                    group: group_i as u8,
                    index: i as u8,
                },
                orbit,
                epoch: Epoch::new(
                    i as f64 / NUM_SATS_PER_GROUP as f64,
                    orbit.period() * TAU * group_i as f64 / (NUM_SATS_PER_GROUP ^ 2) as f64
                ) 
            }
        }).collect()
    }).collect();

    const ORBIT_SAMPLES: u32 = 100;

    let elipses: Vec<Polyline3d> = orbits.iter().map(|orbit| -> Polyline3d {
            let mut verts = orbit.params
                        .generate_points(ORBIT_SAMPLES)
                        .transform(DAffine3::from_scale(DVec3::splat(1.0 / SCALE)))
                        .to_f32();
            verts.push(verts[0]);

            Polyline3d::new(verts)
                .with_color(YELLOW)
                .with_width(0.5)
                .with_perspective(false)
        }).collect();

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

    while window.render_3d(&mut scene, &mut camera).await {
        // Lines only last one frame, so redraw every frame.
        for (dir, color) in axes {
            window.draw_line(-dir * axis_len, dir * axis_len, color, 0.5, false);
        }

        for orbit_elipse in &elipses {
            window.draw_polyline(&orbit_elipse);
        }

        window.draw_point(Vec3 { x: (1.2e7 / SCALE) as f32, y: 0.0, z: 0.0 }, RED, 8.0);

        for group in &groups {
            for sat in group {
                let sat_pos = sat.position(time_elapsed) / SCALE;
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

        // Spin about the world Z (polar) axis: one full turn per sidereal day.
        // Wrap in f64 before casting so the angle keeps f32 precision over long runs.
        let spin = (TAU * time_elapsed / earth.sr_day).rem_euclid(TAU) as f32;
        earth_sphere.set_rotation(Quat::from_axis_angle(Vec3::Z, spin) * earth_base_rot);
    }
    
}
