#[derive(Debug)]
struct OrbitParams {
    semi_maj_axis: f64, // Half of the elipse's major axis diameter
    eccentricity: f64, // 0 - Circular orbit, closer to 1 - stretched elipse
    inclincation: f64, // Inclidation relative to parent equator
    asc_node_lon: f64, // Longitue of the asending node (latitude where orbit crosses equator going up)
    arg_periapsis: f64, // Angle betwee asc_node_lon and periapsis along orbital plane
}

#[derive(Debug)]
struct Planet {
    mass: f64,
    radius: f64,
}

#[derive(Debug)]
struct Orbit<'a> {
    parent: &'a Planet,
    params: OrbitParams,
}

impl<'a> Orbit<'a> {
    fn new(parent: &'a Planet, pe_alt: f64, ap_alt: f64, inclination: f64, asc_node_lon: f64, arg_periapsis: f64) -> Orbit<'a> {
        let periapsis = pe_alt + parent.radius;
        let apoapsis = ap_alt + parent.radius;
        let semi_maj_axis = (periapsis + apoapsis) / 2.0;
        let eccentricity = 1.0 - periapsis / semi_maj_axis;

        Orbit { 
            parent: parent,
            params: OrbitParams {
                semi_maj_axis: semi_maj_axis,
                eccentricity: eccentricity,
                inclincation: inclination,
                asc_node_lon: asc_node_lon,
                arg_periapsis: arg_periapsis
            },
        }
    }
}

#[derive(Debug)]
struct Epoch {
    mean_anomaly: f64, // Position in orbit
    time_0: f64, // Time at which the object is at that position
}

#[derive(Debug)]
struct Sattelite<'a> {
    orbit: Orbit<'a>,
    epoch: Epoch
}

fn main() {
    println!("Hello, world!");
}
