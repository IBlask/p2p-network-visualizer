use std::sync::{Arc, Mutex};
use egui::{Color32, Pos2, Vec2, Visuals};
use serde::Deserialize;
use tokio::sync::mpsc;


#[derive(Clone)]
pub struct MyApp {
    pub(crate) state: Arc<Mutex<State>>,
    pub(crate) nodes_arc: Arc<Mutex<Vec<Node>>>,
    pub(crate) links_arc: Arc<Mutex<Vec<Link>>>,

    pub(crate) left_side_panel_width: f32,
    pub(crate) left_side_panel_color: Color32,
    pub(crate) default_node_color: Color32,
    pub(crate) offline_node_color: Color32,

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
    pub(crate) fn new() -> Self {
        let state = Arc::new(Mutex::new(State::new()));
        
        Self {
            state,
            nodes_arc: Arc::new(Mutex::new(Vec::new())),
            links_arc: Arc::new(Mutex::new(Vec::new())),
            ..Default::default()
        }
    }

    pub(crate) fn set_ctx(&mut self, cc: &eframe::CreationContext<'_>) -> &mut Self {
        self.state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        self
    }

    pub(crate) fn set_dark_theme(&mut self) -> &mut Self {
        let ctx = self.state.lock().unwrap().ctx.clone();
        if ctx.is_some() {
            ctx.unwrap().set_visuals(Visuals::dark());
            self.left_side_panel_color = Color32::from_rgb(40, 40, 40);
            self.default_node_color = Color32::WHITE;
            self.offline_node_color = Color32::DARK_GRAY;
        }
        self
    }

    pub(crate) fn set_light_theme(&mut self) -> &mut Self {
        let ctx = self.state.lock().unwrap().ctx.clone();
        if ctx.is_some() {
            ctx.as_ref().unwrap().set_visuals(Visuals::light());
            self.left_side_panel_color = Color32::from_rgb(200, 200, 200);
            self.default_node_color = Color32::BLACK;
            self.offline_node_color = Color32::LIGHT_GRAY;
        }
        self
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(State::new())),
            nodes_arc: Arc::new(Mutex::new(Vec::new())),
            links_arc: Arc::new(Mutex::new(Vec::new())),

            left_side_panel_width: 180.0,
            left_side_panel_color: Color32::from_rgb(40, 40, 40),
            default_node_color: Color32::WHITE,
            offline_node_color: Color32::DARK_GRAY,

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
    pub(crate) tx: Option<mpsc::Sender<(crate::models::UpdateNodeRequest, tokio::sync::oneshot::Sender<crate::api::ApiResponse>)>>,
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
pub struct Node {
    pub id: String,
    pub name: String,
    pub(crate) center: egui::Pos2,
    pub(crate) radius: f32,
    pub(crate) is_online: bool,
    pub ip_addr: String,
    pub cpu: String,
    pub ram: String,
    pub rom: String,
    pub os: String,
    pub network_bw: String,
    pub software: String,
}

impl Node {
    pub(crate) fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            center: egui::Pos2::default(),
            radius: 0.0,
            is_online: true,
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




#[derive(Deserialize, Debug)]
pub struct UpdateNodeRequest {
    pub node_id: String,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub status: Option<bool>,
    pub ip_addr: Option<String>,
    pub cpu: Option<String>,
    pub ram: Option<String>,
    pub rom: Option<String>,
    pub os: Option<String>,
    pub network_bw: Option<String>,
    pub software: Option<String>,
}

impl Default for UpdateNodeRequest {
    fn default() -> Self {
        Self {
            node_id: String::default(),
            name: None,
            status: None,
            ip_addr: None,
            cpu: None,
            ram: None,
            rom: None,
            os: None,
            network_bw: None,
            software: None,
        }
    }
}