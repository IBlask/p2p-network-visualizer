mod node_status_api;
mod node_attributes_api;

use egui::Color32;
use node_status_api::{node_down, node_up};
use node_attributes_api::node_attributes_update;

use std::{net::SocketAddr, sync::{Arc, Mutex}};
use axum::{routing::post, Json, Router};
use serde::Serialize;
use crate::models::{Node, State, UpdateNodeRequest};


#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}


pub async fn web_server(state: Arc<Mutex<State>>) {
    let state_for_node_down = state.clone();
    let state_for_node_up = state.clone();
    let state_for_node_update = state.clone();

    let app = Router::new()
        .route(
            "/node_down", 
            post(move |payload| node_down(payload, state_for_node_down)))
        .route(
            "/node_up", 
            post(move |payload| node_up(payload, state_for_node_up)))
        .route(
            "/node_update", 
            post(move |payload| node_attributes_update(payload, state_for_node_update)));


    let addr = SocketAddr::from(([0, 0, 0, 0], 8020));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn respond_with(response: ApiResponse) -> Json<ApiResponse> {
    Json(response)
}





pub fn update_node(node: &mut Node, update_request: UpdateNodeRequest, online_color: Color32, offline_color: Color32) {
    if update_request.status.is_some() {
        if update_request.status.unwrap() == true {
            node.color = online_color;
        }
        else if update_request.status.unwrap() == false {
            node.color = offline_color;
        }
    }
    else {
        if let Some(name) = update_request.name {
            node.name = name;
        }
        if let Some(ip_addr) = update_request.ip_addr {
            node.ip_addr = ip_addr;
        }
        if let Some(cpu) = update_request.cpu {
            node.cpu = cpu;
        }
        if let Some(ram) = update_request.ram {
            node.ram = ram;
        }
        if let Some(rom) = update_request.rom {
            node.rom = rom;
        }
        if let Some(os) = update_request.os {
            node.os = os;
        }
        if let Some(network_bw) = update_request.network_bw {
            node.network_bw = network_bw;
        }
        if let Some(software) = update_request.software {
            node.software = software;
        }
    }
}