#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

use eframe::egui;
use egui::{Color32, Stroke};

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn main() -> Result<(), eframe::Error> {
    let mut my_app = MyApp::default();

    // Run eframe
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Vizualizator",
        options,
        Box::new(move |cc| Box::new(MyApp::new(&mut my_app, cc))),
    )
}

struct Node {
    id: String,
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




    fn parse_graphml(&mut self, reader: BufReader<File>) {
        let parser = EventReader::new(reader);
    
        let mut current_node_id = String::new();
        let mut current_node_pos_x: f32 = 0.0;
        let mut current_node_pos_y: f32 = 0.0;
        let mut current_pos: char = 'x';
        let mut current_source = String::new();
        let mut current_target = String::new();
        let mut id_map = HashMap::new();

        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    match name.local_name.as_str() {
                        "node" => {
                            for attr in attributes {
                                if attr.name.local_name == "id" {
                                    current_node_id = attr.value.clone();
                                }
                            }
                        }
                        "data" => {
                            for attr in attributes {
                                if attr.name.local_name == "key" {
                                    if attr.value == "d0" {
                                        current_pos = 'x';
                                    }
                                    else if attr.value == "d1" {
                                        current_pos = 'y';
                                    }
                                }
                            }
                        }
                        "edge" => {
                            for attr in attributes {
                                match attr.name.local_name.as_str() {
                                    "source" => current_source = attr.value.clone(),
                                    "target" => current_target = attr.value.clone(),
                                    _ => {}
                                }
                            }
                            let link_data = Link {
                                node1_index: *id_map.get(&current_source).unwrap_or(&0),
                                node2_index: *id_map.get(&current_target).unwrap_or(&0),
                            };
                            self.links_arc.lock().unwrap().append(&mut vec![link_data]);
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    if current_pos == 'x' {
                        current_node_pos_x = text.parse::<f32>().unwrap();
                    }
                    else if current_pos == 'y' {
                        current_node_pos_y = text.parse::<f32>().unwrap();
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "node" {
                        let node_data = Node {
                            id: current_node_id.clone(),
                            center: egui::pos2(current_node_pos_x, current_node_pos_y),
                            radius: 15.0,
                            color: Color32::WHITE,
                        };
                        self.nodes_arc.lock().unwrap().append(&mut vec![node_data]);
                        id_map.insert(current_node_id.clone(), self.nodes_arc.lock().unwrap().len() - 1);
                    }
                }
                _ => {}
            }
        }

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
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.add(egui::Button::new("Dodaj čvor"));
            ui.add(egui::Button::new("Izbriši čvor"));
            ui.add(egui::Button::new("Dodaj vezu"));
            ui.add(egui::Button::new("Izbriši vezu"));
            ui.add(egui::Button::new("Spremi kao novi preset"));

            if ui.button("Učitaj iz GraphML datoteke").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let path = Some(path.display().to_string());
                    let graphml_file = File::open(path.unwrap()).expect("Otvori GraphML datoteku");
                    let reader = BufReader::new(graphml_file);
                    MyApp::parse_graphml(self, reader);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let nodes_lock = self.nodes_arc.lock().unwrap();
            let links_lock = self.links_arc.lock().unwrap();

            for link in &*links_lock {
                ui.painter().line_segment(
                    [nodes_lock.get(link.node1_index).unwrap().center, nodes_lock.get(link.node2_index).unwrap().center],
                    Stroke::new(1.0, Color32::WHITE),
                );
            }

            for node in &*nodes_lock {
                ui.painter()
                    .circle_filled(node.center, node.radius, node.color);
                let circle_rect = egui::Rect::from_center_size(node.center, (2.0 * node.radius, 2.0 * node.radius).into());
                if circle_rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or_default())) {
                    ui.painter().text(node.center, egui::Align2::CENTER_CENTER, node.id.as_str(), egui::FontId::monospace(node.radius), Color32::BLACK);
                }
            }
        });
    }
}

fn tcp_connections(state: Arc<Mutex<State>>, nodes_arc: Arc<Mutex<Vec<Node>>>, links_arc: Arc<Mutex<Vec<Link>>>) {
    let addr = "127.0.0.1:8020".parse().unwrap();
    let socket = TcpListener::bind(&addr).unwrap();

    tokio::run(socket.incoming().for_each(move |stream| {
        tokio::spawn(handle_connection(stream, state.clone(), nodes_arc.clone(), links_arc.clone()));
        Ok(())
    }).map_err(|e| eprintln!("Error in tcp_connections: {}", e)));
}

fn handle_connection(stream: TcpStream, state: Arc<Mutex<State>>, nodes_arc: Arc<Mutex<Vec<Node>>>, _links_arc: Arc<Mutex<Vec<Link>>>) -> Box<dyn Future<Item = (), Error = ()> + Send> {
    let handle = tokio::io::read_to_end(stream, Vec::new())
        .and_then(move |(_, data)| {
            let received_data_str = String::from_utf8_lossy(&data);

            if let Ok(index) = received_data_str.trim().parse::<usize>() {
                let mut nodes = nodes_arc.lock().unwrap();
                nodes[index].color = Color32::DARK_GRAY;

                let ctx = &state.lock().unwrap().ctx;
                match ctx {
                    Some(x) => x.request_repaint(),
                    None => panic!("error in Option<>"),
                }
            } else {
                eprintln!("Error parsing data as integer");
            }

            Ok(())
        })
        .map_err(|e| {
            eprintln!("Error reading from socket: {}", e);
        });

    Box::new(handle)
}
