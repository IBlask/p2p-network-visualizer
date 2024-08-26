mod nff_utils;
mod api;
mod gui_control;

extern crate tokio;
use std::sync::{Arc, Mutex};

use api::ApiResponse;
use eframe::egui;
use egui::{Color32, Pos2, Vec2};

use gui_control::{setup_side_panel, render_graph};
use tokio::sync::{mpsc, oneshot};


#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let (tx, mut rx) = mpsc::channel::<(String, Color32, oneshot::Sender<ApiResponse>)>(32);
    let state = Arc::new(Mutex::new(State {
        ctx: None,
        tx: Some(tx.clone()),
    }));

    let state_clone = state.clone();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.spawn(async move {
        api::web_server(state_clone).await;
    });

    
    eframe::run_native(
        "Visualizer",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
            ..Default::default()
        },
        Box::new(move |cc| {
            let my_app = MyApp::new(cc);

            // Obrada poruka sa web API-ja
            let nodes_arc = my_app.nodes_arc.clone();
            let state_clone = my_app._state.clone();

            // primanje podataka sa API-ja
            tokio::spawn(async move {
                while let Some((node_id, color, resp_tx)) = rx.recv().await {
                    let mut nodes = nodes_arc.lock().unwrap();
                    
                    if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
                        node.color = color;
                        if let Some(ctx) = &state_clone.lock().unwrap().ctx {
                            ctx.request_repaint();
                        }
                        let _ = resp_tx.send(ApiResponse { success: true, message: "OK".to_string()});
                    } else {
                        let _ = resp_tx.send(ApiResponse { success: false, message: "Čvor nije pronađen".to_string() });
                    }
                }
            });

            Box::new(my_app)
        }),
    )
}

#[derive(Clone)]
struct Node {
    id: String,
    name: String,
    center: egui::Pos2,
    radius: f32,
    color: Color32,
    ip_addr: String,
    cpu: String,
    ram: String,
    rom: String,
    os: String,
    network_bw: String,
    software: String,
}

impl Node {
    fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            center: egui::Pos2::default(),
            radius: 0.0,
            color: egui::Color32::WHITE,
            ip_addr: String::new(),
            cpu: String::new(),
            ram: String::new(),
            rom: String::new(),
            os: String::new(),
            network_bw: String::new(),
            software: String::new(),
        }
    }

    fn set_pos(&mut self, pos: Pos2) -> &mut Self {
        self.center = pos;
        self
    }

    fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }
}


#[derive(Clone)]
struct Link {
    node1_id: String,
    node2_id: String,
}

struct State {
    ctx: Option<egui::Context>,
    tx: Option<mpsc::Sender<(String, Color32, oneshot::Sender<ApiResponse>)>>,
}

impl State {
    pub fn new() -> Self {
        Self { 
            ctx: None,
            tx: None,
        }
    }
}

struct MyApp {
    _state: Arc<Mutex<State>>,
    nodes_arc: Arc<Mutex<Vec<Node>>>,
    links_arc: Arc<Mutex<Vec<Link>>>,

    left_side_panel_width: f32,

    dragging: bool,
    dragged_node_id: Option<String>,
    mouse_drag_delta: Vec2,
    zoom: f32,

    show_error: bool,
    error_message: String,

    show_node_names: bool,
    node_popup: Option<Node>,
    node_default_radius: f32,

    adding_node: bool,
    new_node: Node,
    show_input_dialog: bool,
    new_node_pos: Pos2,

    deleting_node: bool,
    show_delete_dialog: bool,
    node_to_delete: Option<Node>,
    left_click_released: bool,

    adding_link: bool,
    deleting_link: bool,
    first_node_selected: Option<Node>,
    second_node_selected: Option<Node>,

    node_editing: bool,
    node_to_edit: Option<Node>,
    node_to_edit_id: Option<String>,
    node_to_edit_name: Option<String>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _state = Arc::new(Mutex::new(State::new()));
        _state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        
        Self {
            _state,
            nodes_arc: Arc::new(Mutex::new(Vec::new())),
            links_arc: Arc::new(Mutex::new(Vec::new())),
            ..Default::default()
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            _state: Arc::new(Mutex::new(State::new())),
            nodes_arc: Arc::new(Mutex::new(Vec::new())),
            links_arc: Arc::new(Mutex::new(Vec::new())),

            left_side_panel_width: 180.0,

            dragging: false,
            dragged_node_id: None,
            mouse_drag_delta: Vec2::default(),
            zoom: 1.0,

            show_error: false,
            error_message: String::default(),
            
            show_node_names: false,
            node_popup: None,
            node_default_radius: 15.0,

            adding_node: false, 
            new_node: Node::new(),
            show_input_dialog: false,
            new_node_pos: Pos2::default(),

            deleting_node: false,
            show_delete_dialog: false,
            node_to_delete: None,
            left_click_released: true,

            adding_link: false,
            deleting_link: false,
            first_node_selected: None,
            second_node_selected: None,

            node_editing: false,
            node_to_edit: None,
            node_to_edit_id: None,
            node_to_edit_name: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        setup_side_panel(ctx, self);
        render_graph(ctx, self);
    }
}
