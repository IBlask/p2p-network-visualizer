mod tcp_api;
mod web_api;


use std::{net::SocketAddr, sync::{Arc, Mutex}};
use axum::{routing::post, Router};
use serde::Deserialize;
use crate::State;


#[derive(Deserialize, Debug)]
pub struct NodeStatus {
    node_id: String,
}


pub async fn web_server(state: Arc<Mutex<State>>) {
    let state_for_node_down = state.clone();
    let state_for_node_up = state.clone();
    
    let app = Router::new()
        .route(
            "/node_down", 
            post(move |payload| web_api::node_down(payload, state_for_node_down)))
        .route(
            "/node_up", 
            post(move |payload| web_api::node_up(payload, state_for_node_up)));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}