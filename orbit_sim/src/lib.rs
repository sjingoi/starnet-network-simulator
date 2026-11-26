mod orbits;
mod utils;
mod visualizer;
mod constellation;
mod topology;

#[unsafe(no_mangle)]
pub extern "C" fn start_visualizer() {
    visualizer::visualize();
}