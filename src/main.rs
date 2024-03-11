#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::thread;
use std::sync::{Arc, Mutex};

use eframe::egui;
use egui::{Color32, Stroke};

fn main() -> Result<(), eframe::Error> {
    let nodes_arc = Arc::new(Mutex::new(vec![
        Node {
            center: egui::pos2(250.0, 100.0),
            radius: 10.0,
            color: Color32::WHITE,
        },
        Node {
            center: egui::pos2(400.0, 100.0),
            radius: 10.0,
            color: Color32::WHITE,
        },
    ]));

    let links_arc = Arc::new(Mutex::new(vec![
        Link {
            node1: Node {
                center: egui::pos2(250.0, 100.0),
                radius: 10.0,
                color: Color32::WHITE,
            },
            node2: Node {
                center: egui::pos2(400.0, 100.0),
                radius: 10.0,
                color: Color32::WHITE,
            },
        },
    ]));


    // Run eframe
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Vizualizator",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc, nodes_arc, links_arc))),
    )
}

struct Node {
    center: egui::Pos2,
    radius: f32,
    color: Color32,
}

struct Link {
    node1: Node,
    node2: Node,
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
    fn new(cc: &eframe::CreationContext<'_>, nodes_arc: Arc<Mutex<Vec<Node>>>, links_arc: Arc<Mutex<Vec<Link>>>) -> Self {
        let _state = Arc::new(Mutex::new(State::new()));
        _state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        
        let state_clone = _state.clone();
        let nodes_arc_clone = nodes_arc.clone();
        thread::spawn(move || tcp_connections(state_clone, nodes_arc_clone));

        Self { _state, nodes_arc, links_arc }
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
            ui.add(egui::Button::new("Učitaj preset"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            for node in &*self.nodes_arc.lock().unwrap() {
                ui.painter()
                    .circle_filled(node.center, node.radius, node.color);
            }

            for link in &*self.links_arc.lock().unwrap() {
                ui.painter().line_segment(
                    [link.node1.center, link.node2.center],
                    Stroke::new(1.0, Color32::WHITE),
                );
            }
        });
    }
}

fn tcp_connections(state: Arc<Mutex<State>>, nodes_arc: Arc<Mutex<Vec<Node>>>) {
    let addr = "127.0.0.1:8020".parse().unwrap();
    let socket = TcpListener::bind(&addr).unwrap();

    tokio::run(socket.incoming().for_each(move |stream| {
        tokio::spawn(handle_connection(stream, state.clone(), nodes_arc.clone()));
        Ok(())
    }).map_err(|e| eprintln!("Error in tcp_connections: {}", e)));
}

fn handle_connection(stream: TcpStream, state: Arc<Mutex<State>>, nodes_arc: Arc<Mutex<Vec<Node>>>) -> Box<dyn Future<Item = (), Error = ()> + Send> {
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
