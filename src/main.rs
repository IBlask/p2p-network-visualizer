extern crate tokio;
use std::{sync::{Arc, Mutex}, thread};

use eframe::egui;
use egui::Color32;

mod parser;
mod api;
mod control;

use api::tcp_api::tcp_connections;
use control::gui_control::{setup_side_panel, render_graph};

fn main() -> Result<(), eframe::Error> {
    let mut my_app = MyApp::default();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vizualizator",
        options,
        Box::new(move |cc| Box::new(MyApp::new(&mut my_app, cc))),
    )
}

struct Node {
    id: String,
    name: String,
    center: egui::Pos2,
    radius: f32,
    color: Color32,
}

struct Link {
    node1_index: usize,
    node2_index: usize,
}

struct State {
    ctx: Option<egui::Context>,
}

impl State {
    pub fn new() -> Self {
        Self { ctx: None }
    }
}

struct MyApp {
    _state: Arc<Mutex<State>>,
    nodes_arc: Arc<Mutex<Vec<Node>>>,
    links_arc: Arc<Mutex<Vec<Link>>>,
}

impl MyApp {
    fn new(&mut self, cc: &eframe::CreationContext<'_>) -> Self {
        let _state = Arc::new(Mutex::new(State::new()));
        _state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        
        let state_clone = _state.clone();
        let nodes_arc_clone = self.nodes_arc.clone();
        let links_arc_clone = self.links_arc.clone();
    
        thread::spawn(move || tcp_connections(state_clone, nodes_arc_clone, links_arc_clone));
        
        Self { _state, nodes_arc: self.nodes_arc.clone(), links_arc: self.links_arc.clone() }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            _state: Arc::new(Mutex::new(State::new())),
            nodes_arc: Arc::new(Mutex::new(Vec::new())),
            links_arc: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        setup_side_panel(ctx, self);
        render_graph(ctx, self);
    }
}
