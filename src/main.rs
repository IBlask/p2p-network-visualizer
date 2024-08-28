#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use p2p_network_visualizer::init_visualizer;


fn main() {
    let _ = init_visualizer().start_visualizer();
}
