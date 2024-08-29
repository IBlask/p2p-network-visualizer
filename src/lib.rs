pub mod models;
mod nff_utils;
mod api;
mod gui_control;

extern crate tokio;
use crate::models::UpdateNodeRequest;
use api::ApiResponse;
use egui::Color32;
use models::{Link, MyApp, Node, State};
use tokio::sync::{mpsc, oneshot};
use std::{borrow::BorrowMut, sync::{Arc, Mutex}};
use eframe::egui;

use gui_control::{setup_side_panel, render_graph};


pub fn init_visualizer() -> MyApp {
    MyApp::new()
}


impl MyApp {
    pub fn start_visualizer(mut self) -> Result<(), eframe::Error> {
        let (tx, mut rx) = mpsc::channel::<(UpdateNodeRequest, oneshot::Sender<ApiResponse>)>(32);
        let state = Arc::new(Mutex::new(State {
            ctx: None,
            tx: Some(tx.clone()),
        }));

        let state_clone = state.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let runtime_handle = runtime.handle().clone();
        runtime.spawn(async move {
            api::web_server(state_clone).await;
        });

        
        let eframe_result = eframe::run_native(
            "Visualizer",
            eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
                ..Default::default()
            },
            Box::new(move |cc| {
                // Inicijalizacija context-a
                self.set_ctx(cc).set_dark_theme();
                
                // Obrada poruka sa web API-ja
                let nodes_arc = self.nodes_arc.clone();
                let state_clone = self.state.clone();

                // primanje podataka sa API-ja
                runtime_handle.spawn(async move {
                    while let Some((update_node_request, resp_tx)) = rx.recv().await {
                        let mut nodes = nodes_arc.lock().unwrap();
                        
                        if let Some(node) = nodes.iter_mut().find(|n| n.id == update_node_request.node_id) {
                            api::update_node(node, update_node_request);

                            if let Some(ctx) = &state_clone.lock().unwrap().ctx {
                                ctx.request_repaint();
                            }
                            let _ = resp_tx.send(ApiResponse { success: true, message: "OK".to_string()});
                        } else {
                            let _ = resp_tx.send(ApiResponse { success: false, message: "Čvor nije pronađen".to_string() });
                        }
                    }
                });

                Box::new(self)
            })
        );

        runtime.shutdown_timeout(std::time::Duration::from_secs(1));
        eframe_result
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        setup_side_panel(ctx, self);
        render_graph(ctx, self);
    }
}




impl MyApp {
    pub fn node_down(&self, node_id: String) -> Result<(), String> {
        let mut nodes = self.nodes_arc.lock().unwrap();
        let node = nodes.iter_mut().find(|n| n.id == node_id);
        if node.is_some() {
            node.unwrap().color = Color32::DARK_GRAY;
            self.repaint();
            Ok(())
        }
        else {
            Err("Čvor nije pronađen".to_string())
        }
    }

    pub fn node_up(&self, node_id: String) -> Result<(), String> {
        let mut nodes = self.nodes_arc.lock().unwrap();
        let node = nodes.iter_mut().find(|n| n.id == node_id);
        if node.is_some() {
            node.unwrap().color = Color32::WHITE;
            self.repaint();
            Ok(())
        }
        else {
            Err("Čvor nije pronađen".to_string())
        }
    }

    pub fn change_node_attributes(&self, update_node_request: UpdateNodeRequest) -> Result<(), String> {
        if update_node_request.color.is_some() {
            return Err("Nije moguće mijenjati boju čvora".to_string());
        }

        let mut nodes = self.nodes_arc.lock().unwrap();
        let node = nodes.iter_mut().find(|n| n.id == update_node_request.node_id);
        if node.is_some() {
            api::update_node(node.unwrap().borrow_mut(), update_node_request);
            self.repaint();
            Ok(())
        }
        else {
            Err("Čvor nije pronađen".to_string())
        }
    }


    fn repaint(&self) {
        if let Some(ctx) = &self.state.lock().unwrap().ctx {
            ctx.request_repaint();
        }
    }
}