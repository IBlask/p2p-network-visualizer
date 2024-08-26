mod node_status_api;
mod node_attributes_api;

use node_status_api::{node_down, node_up};
use node_attributes_api::node_attributes_update;

use std::{net::SocketAddr, sync::{Arc, Mutex}};
use axum::{routing::post, Json, Router};
use egui::Color32;
use serde::{Deserialize, Serialize};
use crate::models::{Node, State};


#[derive(Deserialize, Debug)]
pub struct UpdateNodeRequest {
    pub node_id: String,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub color: Option<Color32>,
    pub ip_addr: Option<String>,
    pub cpu: Option<String>,
    pub ram: Option<String>,
    pub rom: Option<String>,
    pub os: Option<String>,
    pub network_bw: Option<String>,
    pub software: Option<String>,
}

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





pub fn update_node(node: &mut Node, update_request: UpdateNodeRequest) {
    if update_request.color.is_some() {
        node.color = update_request.color.unwrap();
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