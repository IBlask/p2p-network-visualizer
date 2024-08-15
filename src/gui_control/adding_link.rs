use crate::{Link, MyApp, Node};

use egui::{Color32, Pos2};


pub fn adding_link(ui: &egui::Ui, ctx: &egui::Context, app: &mut MyApp) {
    let n = Node {
        id: String::from(""), 
        name: String::from(""), 
        center: Pos2::default(), 
        radius: 0.0, 
        color: Color32::default()
    };

    let first_node = app.first_node_selected.clone().unwrap_or(n.clone());
    let second_node = app.second_node_selected.clone().unwrap_or(n);

    if app.first_node_selected.is_some() {
        ui.painter().circle_filled(
            first_node.center, 
            first_node.radius,
            Color32::YELLOW);
    }
    if app.second_node_selected.is_some() {
        ui.painter().circle_filled(
            second_node.center,
            second_node.radius,
            Color32::YELLOW);
    }

    egui::Window::new("Dodaj novu vezu")
        .collapsible(true)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Prvi čvor");
            ui.label(
                format!("    ID: {}\n    Naziv: {}", 
                first_node.id, 
                first_node.name));
            ui.add_space(17.5);

            ui.label("Drugi čvor");
            ui.label(
                format!("    ID: {}\n    Naziv: {}", 
                second_node.id, 
                second_node.name));
            ui.add_space(17.5);
            
            ui.horizontal(|ui| {
                if ui.button("Dodaj vezu").clicked() 
                    && app.first_node_selected.is_some() 
                    && app.second_node_selected.is_some() {

                    let new_link = Link {
                        node1_id: app.first_node_selected.as_ref().unwrap().id.clone(),
                        node2_id: app.second_node_selected.as_ref().unwrap().id.clone(),
                    };
                    let mut links_lock = app.links_arc.lock().unwrap();
                    links_lock.push(new_link);

                    app.adding_link = false;
                    app.first_node_selected = None;
                    app.second_node_selected = None;
                }

                if ui.button("Odustani").clicked() {
                    app.adding_link = false;
                    app.first_node_selected = None;
                    app.second_node_selected = None;
                }
            });
        });
}