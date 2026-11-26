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

/// One-way latency (ms) of the direct link from `source` to `target` at `time_s` (seconds).
/// The two nodes must be neighbours.
#[unsafe(no_mangle)]
pub extern "C" fn sim_latency_ms(_source: Address, _target: Address, _time_s: f64) -> f64 {
    0.0 // 0ms
}

/// Packet loss of the direct link from `source` to `target` at `time_s` (seconds),
/// as a fraction from 0.0 (no loss) to 1.0 (all lost). The two nodes must be neighbours.
#[unsafe(no_mangle)]
pub extern "C" fn sim_packet_loss(_source: Address, _target: Address, _time_s: f64) -> f64 {
    0.0 // no loss
}

/// Returns the neighbour of `node` that a packet headed for `dest` should be forwarded to at `time_s` (seconds).
#[unsafe(no_mangle)]
pub extern "C" fn next_hop(_node: Address, _dest: Address, _time_s: f64) -> Address {
    // Forward to a random node
    let group = rand::random_range(0..NUM_GROUPS);
    let sat = rand::random_range(0..NUM_SAT_PER_GROUP);

    u32::from_be_bytes([0, 0, group, sat])
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_all_nodes(out: *mut Address, capacity: usize) -> usize {
    // Hardcoded stub: three nodes.
    const ADDRESSES: [Address; 3] = [
        u32::from_be_bytes([0, 0, 0, 0]),
        u32::from_be_bytes([0, 0, 0, 1]),
        u32::from_be_bytes([0, 0, 1, 0]),
    ];

    if !out.is_null() {
        let n = capacity.min(ADDRESSES.len());
        unsafe { std::ptr::copy_nonoverlapping(ADDRESSES.as_ptr(), out, n) };
    }

    ADDRESSES.len()
}
