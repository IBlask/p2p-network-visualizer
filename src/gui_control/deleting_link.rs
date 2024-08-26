use crate::models::{Link, MyApp, Node};

use egui::Color32;


pub fn deleting_link(ui: &egui::Ui, ctx: &egui::Context, app: &mut MyApp) {
    let n = Node::new();

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

    egui::Window::new("Izbriši vezu")
        .collapsible(true)
        .resizable(false)
        .default_pos(egui::pos2(app.left_side_panel_width + 10.0, 10.0))
        .show(ctx, |ui| {
            ui.label("Prvi čvor");
            ui.label(
                format!("    ID: {}\n    Naziv: {}", 
                first_node.id, 
                first_node.name));
            ui.add_space(10.0);

            ui.label("Drugi čvor");
            ui.label(
                format!("    ID: {}\n    Naziv: {}", 
                second_node.id, 
                second_node.name));
            ui.add_space(6.0);
            
            let mut link_to_delete: Option<Link> = None;
            if app.first_node_selected.is_some() && app.second_node_selected.is_some() {
                let links_lock = app.links_arc.lock().unwrap().clone();
                link_to_delete = links_lock.iter().find(|l| 
                    (l.node1_id == first_node.id && l.node2_id == second_node.id) || 
                    (l.node1_id == second_node.id && l.node2_id == first_node.id)).cloned();
                drop(links_lock);
            }

            if link_to_delete.is_none() && app.first_node_selected.is_some() && app.second_node_selected.is_some() {
                ui.colored_label(Color32::RED, "Ne postoji veza između odabranih čvorova!");
            }
            else {
                ui.add_space(17.5);
            }

            ui.add_space(6.0);

            ui.horizontal(|ui| {
                if ui.button("Izbriši vezu").clicked() 
                    && app.first_node_selected.is_some() 
                    && app.second_node_selected.is_some() 
                    && link_to_delete.is_some() {

                    let mut links_lock = app.links_arc.lock().unwrap();
                    links_lock.retain(|l| {
                        !(
                            (l.node1_id == app.first_node_selected.as_ref().unwrap().id
                            && l.node2_id == app.second_node_selected.as_ref().unwrap().id)
                            ||
                            (l.node1_id == app.second_node_selected.as_ref().unwrap().id
                            && l.node2_id == app.first_node_selected.as_ref().unwrap().id)
                        )
                    });

                    app.deleting_link = false;
                    app.first_node_selected = None;
                    app.second_node_selected = None;
                }

                if ui.button("Odustani").clicked() {
                    app.deleting_link = false;
                    app.first_node_selected = None;
                    app.second_node_selected = None;
                }
            });
        });
}