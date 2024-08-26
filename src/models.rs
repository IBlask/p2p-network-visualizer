use std::sync::{Arc, Mutex};
use egui::{Color32, Pos2, Vec2};
use tokio::sync::mpsc;


pub(crate) struct MyApp {
    pub(crate) _state: Arc<Mutex<State>>,
    pub(crate) nodes_arc: Arc<Mutex<Vec<Node>>>,
    pub(crate) links_arc: Arc<Mutex<Vec<Link>>>,

    pub(crate) left_side_panel_width: f32,

    pub(crate) dragging: bool,
    pub(crate) dragged_node_id: Option<String>,
    pub(crate) mouse_drag_delta: Vec2,
    pub(crate) zoom: f32,

    pub(crate) show_error: bool,
    pub(crate) error_message: String,

    pub(crate) show_node_names: bool,
    pub(crate) node_popup: Option<Node>,
    pub(crate) node_default_radius: f32,

    pub(crate) adding_node: bool,
    pub(crate) new_node: Node,
    pub(crate) show_input_dialog: bool,
    pub(crate) new_node_pos: Pos2,

    pub(crate) deleting_node: bool,
    pub(crate) show_delete_dialog: bool,
    pub(crate) node_to_delete: Option<Node>,
    pub(crate) left_click_released: bool,

    pub(crate) adding_link: bool,
    pub(crate) deleting_link: bool,
    pub(crate) first_node_selected: Option<Node>,
    pub(crate) second_node_selected: Option<Node>,

    pub(crate) node_editing: bool,
    pub(crate) node_to_edit: Option<Node>,
    pub(crate) node_to_edit_id: Option<String>,
    pub(crate) node_to_edit_name: Option<String>,
}

impl MyApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
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




pub(crate) struct State {
    pub(crate) ctx: Option<egui::Context>,
    pub(crate) tx: Option<mpsc::Sender<(crate::api::UpdateNodeRequest, tokio::sync::oneshot::Sender<crate::api::ApiResponse>)>>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self { 
            ctx: None,
            tx: None,
        }
    }
}




#[derive(Clone)]
pub(crate) struct Node {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) center: egui::Pos2,
    pub(crate) radius: f32,
    pub(crate) color: Color32,
    pub(crate) ip_addr: String,
    pub(crate) cpu: String,
    pub(crate) ram: String,
    pub(crate) rom: String,
    pub(crate) os: String,
    pub(crate) network_bw: String,
    pub(crate) software: String,
}

impl Node {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn set_pos(&mut self, pos: Pos2) -> &mut Self {
        self.center = pos;
        self
    }

    pub(crate) fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }
}




#[derive(Clone)]
pub(crate) struct Link {
    pub(crate) node1_id: String,
    pub(crate) node2_id: String,
}