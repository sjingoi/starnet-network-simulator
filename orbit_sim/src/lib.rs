use crate::{constellation::{Address}, visualizer::{NUM_GROUPS, NUM_SAT_PER_GROUP}};

mod orbits;
mod utils;
mod visualizer;
mod constellation;
mod topology;

#[unsafe(no_mangle)]
pub extern "C" fn start_visualizer() {
    visualizer::visualize();
}

#[unsafe(no_mangle)]
pub extern "C" fn set_visualizer_time(_time_s: f64) {
    // Its possible that we let NS3 control time for the simulator, and have it tell the visualizer
    // the current time. The visualizer will continue to run even without NS3 telling it the time by just
    // running the simulation the way it already does.
}

#[unsafe(no_mangle)]
pub extern "C" fn sim_latency_ms(_source: Address, _target: Address, _time_s: f64) -> f64 {
    0.0 // 0ms
}

#[unsafe(no_mangle)]
pub extern "C" fn sim_packet_loss(_source: Address, _target: Address, _time_s: f64) -> f64 {
    0.0 // 0%
}

#[unsafe(no_mangle)]
pub extern "C" fn next_hop(_node: Address, _dest: Address, _time_s: f64) -> Address {
    // Forward to a random node
    let group = rand::random_range(0..NUM_GROUPS);
    let sat = rand::random_range(0..NUM_SAT_PER_GROUP);

    u32::from_be_bytes([0, 0, group, sat])
}