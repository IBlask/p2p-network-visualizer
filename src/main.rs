#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate tokio;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::thread;

use eframe::egui;
use egui::{Color32, Stroke};

fn main() -> Result<(), eframe::Error> {
    thread::spawn(move || tcp_connections());

    let node1 = Node{center: egui::pos2(250.0, 100.0), radius: 10.0, color: Color32::WHITE};
    let node2 = Node{center: egui::pos2(400.0, 100.0), radius: 10.0, color: Color32::WHITE};
    
    unsafe {
        NODES.push(Node{center: egui::pos2(250.0, 100.0), radius: 10.0, color: Color32::WHITE});
        NODES.push(Node{center: egui::pos2(400.0, 100.0), radius: 10.0, color: Color32::WHITE});
        LINKS.push(Link{node1, node2});
    };


    MyApp::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Vizualizator",
        options,
        Box::new(|_cc| {
            Box::<MyApp>::default()
        }),
    )
}



struct Node {
    center: egui::Pos2,
    radius: f32,
    color: Color32,
}

static mut NODES: Vec<Node> = Vec::new();



struct Link {
    node1: Node,
    node2: Node,
}

static mut LINKS: Vec<Link> = Vec::new();



struct MyApp {
}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
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
            ui.add(egui::Button::new( "Učitaj preset"));
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            let nodes = unsafe { &NODES };
            for node in nodes {
                ui.painter().circle_filled(node.center, node.radius, node.color);
            }

            let links = unsafe { &LINKS };
            for link in links {
                ui.painter().line_segment([link.node1.center, link.node2.center], Stroke::new(1.0, Color32::WHITE));
            }
        });
    }
}



fn tcp_connections() {
    let addr = "127.0.0.1:8020".parse().unwrap();
    let socket = TcpListener::bind(&addr).unwrap();

    tokio::run(socket.incoming().for_each(|stream| {
        tokio::spawn(handle_connection(stream));
        Ok(())
    }).map_err(|e| eprintln!("Error in tcp_connections: {}", e)));

}



fn handle_connection(stream: TcpStream) -> Box<dyn Future<Item = (), Error = ()> + Send> {    
    let handle = tokio::io::read_to_end(stream, Vec::new())
        .and_then(move |(_, data)| {
            let received_data_str = String::from_utf8_lossy(&data);

            if let Ok(index) = received_data_str.trim().parse::<usize>() {
                unsafe { NODES[index].color = Color32::DARK_GRAY; }
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