mod tcp_api;
mod web_api;


use std::{net::SocketAddr, sync::{Arc, Mutex}};
use axum::{routing::post, Router};
use serde::Deserialize;
use crate::State;

use web_api::node_down;


#[derive(Deserialize, Debug)]
pub struct NodeStatus {
    node_id: String,
}


pub async fn web_server(state: Arc<Mutex<State>>) {
    let app = Router::new().route(
        "/node_down",
        post(move |payload| node_down(payload, state.clone())),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}