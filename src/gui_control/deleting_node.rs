use crate::models::MyApp;

use egui::{Color32, Ui};


pub fn show_delete_dialog(ui: &Ui, ctx: &egui::Context, app: &mut MyApp) {
    let node_to_delete = app.node_to_delete.clone().unwrap();
    ui.painter().circle_filled(node_to_delete.center, node_to_delete.radius, Color32::RED);

    if app.left_click_released {
        // lijeva tipka miša više nije stisnuta

        egui::Window::new("Potvrda brisanja čvora")
        .collapsible(true)
        .resizable(false)
        .default_pos(egui::pos2(app.left_side_panel_width + 10.0, 10.0))
        .show(ctx, |ui| {
            ui.label(format!("ID: {}", node_to_delete.id));
            ui.label(format!("Naziv: {}", node_to_delete.name));
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Izbriši").clicked() {
                    let mut nodes_lock = app.nodes_arc.lock().unwrap();
                    let mut links_lock = app.links_arc.lock().unwrap();

                    // Izbriši čvor i veze
                    if let Some(node_index) = nodes_lock.iter().position(|n| n.id == node_to_delete.id) {
                        nodes_lock.remove(node_index);

                        links_lock.retain(|link| {
                            link.node1_id != node_to_delete.id && link.node2_id != node_to_delete.id
                        });
                    }

                    app.show_delete_dialog = false;
                    app.node_to_delete = None;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Odustani").clicked() {
                        app.show_delete_dialog = false;
                    }
                });
            });
        });
    }
}