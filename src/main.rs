extern crate tokio;
use std::{sync::{Arc, Mutex}, thread};

use eframe::egui;
use egui::{Color32, Pos2};

mod parser;
mod api;
mod gui_control;

use api::tcp_api::tcp_connections;
use gui_control::{setup_side_panel, render_graph};

fn main() -> Result<(), eframe::Error> {
    let mut my_app = MyApp::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Vizualizator",
        options,
        Box::new(move |cc| Box::new(MyApp::new(&mut my_app, cc))),
    )
}

#[derive(Clone)]
struct Node {
    id: String,
    name: String,
    center: egui::Pos2,
    radius: f32,
    color: Color32,
}

#[derive(Clone)]
struct Link {
    node1_id: String,
    node2_id: String,
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

    node_popup_name: Option<String>,

    adding_node: bool,
    show_input_dialog: bool,
    new_node_id: String,
    new_node_name: String,
    new_node_pos: Pos2,

    deleting_node: bool,
    show_delete_dialog: bool,
    node_to_delete: Option<Node>,
    left_click_released: bool,
}

impl MyApp {
    fn new(&mut self, cc: &eframe::CreationContext<'_>) -> Self {
        let _state = Arc::new(Mutex::new(State::new()));
        _state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        
        let state_clone = _state.clone();
        let nodes_arc_clone = self.nodes_arc.clone();
        let links_arc_clone = self.links_arc.clone();
    
        thread::spawn(move || tcp_connections(state_clone, nodes_arc_clone, links_arc_clone));
        
        MyApp::default()
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            _state: Arc::new(Mutex::new(State::new())),
            nodes_arc: Arc::new(Mutex::new(Vec::new())),
            links_arc: Arc::new(Mutex::new(Vec::new())),
            node_popup_name: None,
            adding_node: false, 
            show_input_dialog: false,  
            new_node_id: String::new(), 
            new_node_name: String::new(),
            new_node_pos: Pos2::default(),
            deleting_node: false,
            show_delete_dialog: false,
            node_to_delete: None,
            left_click_released: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        setup_side_panel(ctx, self);
        render_graph(ctx, self);
    }
}
