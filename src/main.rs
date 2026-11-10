mod orbits;
mod utils;
use kiss3d::{camera::OrbitCamera3d, color::{BLUE, GREEN, RED, YELLOW}, glamx::{Quat, Vec3}, renderer::Polyline3d, scene::SceneNode3d, window::Window};
use orbits::{Planet, Orbit, Sattelite, Epoch};
use std::{f64::consts::PI, path::Path};

const SCALE: f32 = 1e6;

#[kiss3d::main]
async fn main() {

    let earth = Planet {
        mass: 5.9722e24,
        radius: 6.371e6,
    };

    let group1_orbit = Orbit::new(
        &earth, 
        200000.0, // 200km
        80000000.0, // 8000km
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


    // Plot
    let mut window = Window::new("Orbit shape").await;
    window.set_ambient(1.0);
    // Start at an angle: with Z as the up axis, an eye directly on +Z would be
    // looking straight down the rotation axis (a degenerate view).
    let mut camera = OrbitCamera3d::new(Vec3::new(20.0, -20.0, 15.0), Vec3::ZERO);
    // Make Z "up" so dragging left/right spins the view around the Z axis.
    camera.set_up_axis(Vec3::Z);
    let mut scene = SceneNode3d::empty();

    let mut earth_3d = scene
        .add_sphere(earth.radius as f32 / SCALE)
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

    let mut verts: Vec<Vec3> = points
        .iter()
        .map(|p| Vec3::new(p.x as f32 / SCALE, p.y as f32 / SCALE, p.z as f32 / SCALE))
        .collect();
    verts.push(verts[0]); // close the loop (replaces the wrap-around branch)


    let orbit_elipse = Polyline3d::new(verts)
        .with_color(YELLOW)
        .with_width(0.5)
        .with_perspective(false);

    while window.render_3d(&mut scene, &mut camera).await {
        // Lines only last one frame, so redraw every frame.
        for (dir, color) in axes {
            window.draw_line(-dir * axis_len, dir * axis_len, color, 0.5, false);
        }

        window.draw_polyline(&orbit_elipse);
    }
    
}
