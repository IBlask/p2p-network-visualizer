use std::sync::{Arc, Mutex};
use axum::Json;
use tokio::sync::oneshot;
use crate::State;

use super::{respond_with, ApiResponse, UpdateNodeRequest};


pub async fn node_attributes_update(
    Json(update_node_request): Json<UpdateNodeRequest>,
    state: Arc<Mutex<State>>,
) -> Json<ApiResponse> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = state.lock().unwrap().tx.clone();

    if let Some(tx) = tx {
        if tx.send((update_node_request, resp_tx)).await.is_ok() {
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