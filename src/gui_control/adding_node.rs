use crate::{MyApp, Node};

use egui::{Color32, Pos2, Ui};


pub fn adding_node(ui: &Ui, ctx: &egui::Context, app: &mut MyApp, mouse_pos: Pos2) {
    ui.painter().circle_filled(mouse_pos, app.node_default_radius, Color32::LIGHT_BLUE); 
    if ctx.input(|i| i.pointer.primary_clicked()) {
        app.new_node_pos = mouse_pos;
        app.show_input_dialog = true;
        app.adding_node = false; 
    }
}


pub fn show_input_dialog(ui: &Ui, ctx: &egui::Context, app: &mut MyApp, nodes: &Vec<Node>) {
    ui.painter().circle_filled(app.new_node_pos, app.node_default_radius, Color32::LIGHT_BLUE);

    egui::Window::new("Detalji novog čvora")
        .collapsible(true)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Unesite ID čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node_id).desired_width(ui.available_width()));

            ui.label("Unesite naziv čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node_name).desired_width(ui.available_width()));

            ui.add_space(4.0);

            // Provjera postoji li već čvor s istim ID-jem ili nazivom
            let id_exists = nodes.iter().any(|node| node.id == app.new_node_id);
            if id_exists {
                ui.colored_label(egui::Color32::RED, "ID čvora mora biti jedinstven! Uneseni ID već postoji.");
            }
            else if nodes.iter().any(|node| node.name == app.new_node_name) {
                ui.colored_label(egui::Color32::RED, "Oprez! Već postoji čvor s istim nazivom.");
            }
            else {
                ui.add_space(17.5);
            }

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    if !app.new_node_id.is_empty() && !app.new_node_name.is_empty() && !id_exists {
                        let new_node = crate::Node {
                            id: app.new_node_id.clone(),
                            name: app.new_node_name.clone(),
                            center: app.new_node_pos,
                            radius: app.node_default_radius,
                            color: Color32::WHITE,
                        };
                        let mut nodes_lock = app.nodes_arc.lock().unwrap();
                        nodes_lock.push(new_node);

                        app.show_input_dialog = false;
                        app.new_node_id = String::default();
                        app.new_node_name = String::default();
                        app.new_node_pos = Pos2::default();
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Odustani").clicked() {
                        app.show_input_dialog = false;
                    }
                });
            });
        });
}