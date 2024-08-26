use axum::Json;
use tokio::sync::oneshot;
use std::sync::{Arc, Mutex};
use crate::State;
use egui::Color32;

use super::{respond_with, ApiResponse, NodeStatus};

pub async fn node_down(
    Json(node_status): Json<NodeStatus>,
    state: Arc<Mutex<State>>,
) -> Json<ApiResponse> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = state.lock().unwrap().tx.clone();

    if let Some(tx) = tx {
        if tx.send((node_status.node_id.clone(), Color32::DARK_GRAY, resp_tx)).await.is_ok() {
            respond_with(resp_rx.await.unwrap()).await
        } else {
            respond_with(ApiResponse{
                success: false,
                message: "Greška u obradi zahtjeva. Pokušajte ponovno.".to_string()
            }).await
        }
    } else {
        respond_with(ApiResponse{
            success: false,
            message: "Interna greška. Ponovno pokrenite vizualizator.".to_string()
        }).await
    }
}


pub async fn node_up(
    Json(node_status): Json<NodeStatus>,
    state: Arc<Mutex<State>>,
) -> Json<ApiResponse> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = state.lock().unwrap().tx.clone();

    if let Some(tx) = tx {
        if tx.send((node_status.node_id.clone(), Color32::WHITE, resp_tx)).await.is_ok() {
            respond_with(resp_rx.await.unwrap()).await
        } else {
            respond_with(ApiResponse{
                success: false,
                message: "Greška u obradi zahtjeva. Pokušajte ponovno.".to_string()
            }).await
        }
    } else {
        respond_with(ApiResponse{
            success: false,
            message: "Interna greška. Ponovno pokrenite vizualizator.".to_string()
        }).await
    }
}

