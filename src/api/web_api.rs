use axum::extract::Json;
use std::sync::{Arc, Mutex};
use crate::State;
use egui::Color32;

use super::NodeStatus;


pub async fn node_down(
    Json(node_status): Json<NodeStatus>,
    state: Arc<Mutex<State>>
) {
    let tx = state.lock().unwrap().tx.clone();
    if let Some(tx) = tx {
        let _ = tx.send((node_status.node_id, Color32::DARK_GRAY)).await;
    }
}


pub async fn node_up(
    Json(node_status): Json<NodeStatus>,
    state: Arc<Mutex<State>>
) {
    let tx = state.lock().unwrap().tx.clone();
    if let Some(tx) = tx {
        let _ = tx.send((node_status.node_id, Color32::WHITE)).await;
    }
}
