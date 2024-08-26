use crate::models::{MyApp, Node};

use egui::{Color32, Pos2, Ui};

use super::painter::scale_pos;


pub fn adding_node(ui: &Ui, ctx: &egui::Context, app: &mut MyApp, mouse_pos: Pos2) {
    ui.painter().circle_filled(mouse_pos, app.node_default_radius, Color32::LIGHT_BLUE); 
    if ctx.input(|i| i.pointer.primary_clicked()) {
        app.new_node_pos = (mouse_pos - app.mouse_drag_delta) / app.zoom;
        app.show_input_dialog = true;
        app.adding_node = false; 
    }
}


pub fn show_input_dialog(ui: &Ui, ctx: &egui::Context, app: &mut MyApp, nodes: &Vec<Node>) {
    ui.painter().circle_filled(scale_pos(app.new_node_pos, app), app.node_default_radius, Color32::LIGHT_BLUE);

    egui::Window::new("Detalji novog čvora")
        .collapsible(true)
        .resizable(false)
        .default_pos(egui::pos2(app.left_side_panel_width + 10.0, 10.0))
        .show(ctx, |ui| {
            ui.label("Unesite ID čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.id).desired_width(ui.available_width()));

            ui.label("Unesite naziv čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.name).desired_width(ui.available_width()));

            ui.label("Unesite IP adresu čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.ip_addr).desired_width(ui.available_width()));

            ui.label("Unesite CPU čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.cpu).desired_width(ui.available_width()));

            ui.label("Unesite RAM čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.ram).desired_width(ui.available_width()));

            ui.label("Unesite ROM čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.rom).desired_width(ui.available_width()));

            ui.label("Unesite OS čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.os).desired_width(ui.available_width()));

            ui.label("Unesite propusnost mreže čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.network_bw).desired_width(ui.available_width()));

            ui.label("Unesite softver čvora:");
            ui.add(egui::TextEdit::singleline(&mut app.new_node.software).desired_width(ui.available_width()));

            ui.add_space(4.0);

            // Provjera postoji li već čvor s istim ID-jem ili nazivom
            let id_exists = nodes.iter().any(|node| node.id == app.new_node.id);
            if id_exists {
                ui.colored_label(egui::Color32::RED, "ID čvora mora biti jedinstven! Uneseni ID već postoji.");
            }
            else if nodes.iter().any(|node| node.name == app.new_node.name) {
                ui.colored_label(egui::Color32::RED, "Oprez! Već postoji čvor s istim nazivom.");
            }
            else {
                ui.add_space(17.5);
            }

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    if !app.new_node.id.is_empty() && !app.new_node.name.is_empty() && !id_exists {
                        app.new_node.set_pos(app.new_node_pos)
                                    .set_radius(app.node_default_radius);

                        if let Ok(mut nodes_lock) = app.nodes_arc.lock() {
                            
                            nodes_lock.push(app.new_node.clone());

                            app.show_input_dialog = false;
                            app.new_node_pos = Pos2::default();
                        } else {
                            ui.add_space(8.0);
                            ui.colored_label(Color32::RED, "Došlo je do greške. Pokušajte ponovno!");
                        }                        
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