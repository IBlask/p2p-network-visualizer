use std::sync::{Arc, Mutex};

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use crate::{Node, Link, State};

pub fn tcp_connections(state: Arc<Mutex<State>>, nodes_arc: Arc<Mutex<Vec<Node>>>, links_arc: Arc<Mutex<Vec<Link>>>) {
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
                nodes[index].color = egui::Color32::DARK_GRAY;

                let ctx = &state.lock().unwrap().ctx;
                if let Some(x) = ctx {
                    x.request_repaint();
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
