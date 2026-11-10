mod orbits;
mod utils;
use kiss3d::{camera::OrbitCamera3d, color::{BLUE, GREEN, RED, YELLOW}, glamx::{Quat, Vec3}, renderer::Polyline3d, scene::SceneNode3d, window::Window};
use orbits::{Planet, Orbit};
use std::{f64::consts::{PI, TAU}, path::Path};
use glam::{DAffine3, DVec3};

use crate::utils::VecDVec3Ext;

const SCALE: f64 = 1e6;

#[kiss3d::main]
async fn main() {

    let earth = Planet {
        mass: 5.9722e24,
        radius: 6.371e6,
    };

    const NUM_ORBITS: u32 = 12;

    let orbits: Vec<Orbit> = (0..NUM_ORBITS).map(|i| Orbit::new(
        &earth, 
        1000000.0, // 1000km
        1000000.0, // 1000km
        3.0 * PI/8.0, 
        TAU * i as f64 / NUM_ORBITS as f64,
        0.0,
    )).collect();

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
    let mut camera = OrbitCamera3d::new(Vec3::new(20.0, -20.0, 15.0), Vec3::ZERO);
    // Make Z "up" so dragging left/right spins the view around the Z axis.
    camera.set_up_axis(Vec3::Z);
    let mut scene = SceneNode3d::empty();

    scene.add_sphere((earth.radius / SCALE) as f32)
        .set_texture_from_file(Path::new("assets/2k_earth_daymap.jpg"), "earth")
        // kiss3d's sphere has its poles on Y, with the image's top row (north) at -Y.
        // Rotating -90° about X sends -Y to +Z, so north points up the Z axis.
        .set_rotation(Quat::from_axis_angle(Vec3::X, -std::f32::consts::FRAC_PI_2));

    // Axis lines run from -AXIS_LEN to +AXIS_LEN; long enough to reach past the orbit.
    let axis_len: f32 = 150.0;
    let axes = [
        (Vec3::X, RED),
        (Vec3::Y, GREEN),
        (Vec3::Z, BLUE),
    ];

    while window.render_3d(&mut scene, &mut camera).await {
        // Lines only last one frame, so redraw every frame.
        for (dir, color) in axes {
            window.draw_line(-dir * axis_len, dir * axis_len, color, 0.5, false);
        }

        for orbit_elipse in &elipses {
            window.draw_polyline(&orbit_elipse);
        }
    }
    
}
