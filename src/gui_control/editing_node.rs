use egui::{Color32, Ui};

use crate::models::MyApp;

pub fn show_node_editing_dialog(ui: &Ui, ctx: &egui::Context, app: &mut MyApp) {
    let mut node = app.node_to_edit.clone().unwrap();
    ui.painter().circle_filled(node.center, node.radius, Color32::BLUE);

    egui::Window::new("Uredi detalje čvora")
        .collapsible(false)
        .resizable(false)
        .default_pos(egui::pos2(app.left_side_panel_width + 10.0, 10.0))
        .show(ctx, |ui| {
            if app.node_to_edit_id.is_none() {
                app.node_to_edit_id = Some(node.id.clone());
                app.node_to_edit_name = Some(node.name.clone());
            }

            ui.label("ID:");
            ui.text_edit_singleline(&mut node.id);

            ui.label("Naziv:");
            ui.text_edit_singleline(&mut node.name);

            ui.label("IP adresa:");
            ui.text_edit_singleline(&mut node.ip_addr);

            ui.label("CPU:");
            ui.text_edit_singleline(&mut node.cpu);

            ui.label("RAM:");
            ui.text_edit_singleline(&mut node.ram);

            ui.label("ROM:");
            ui.text_edit_singleline(&mut node.rom);

            ui.label("OS:");
            ui.text_edit_singleline(&mut node.os);

            ui.label("Mrežna propusnost:");
            ui.text_edit_singleline(&mut node.network_bw);

            ui.label("Software:");
            ui.text_edit_singleline(&mut node.software);

            ui.add_space(4.0);

            // Provjera postoji li već čvor s istim ID-jem
            let nodes_lock = app.nodes_arc.lock().unwrap();
            let id_exists = node.id != app.node_to_edit_id.clone().unwrap() && nodes_lock.iter().any(|n| n.id == node.id);
            if id_exists {
                ui.colored_label(Color32::RED, "Već postoji čvor s tom ID oznakom. ID mora biti jedinstven za svaki čvor!");
            }
            else if node.name != app.node_to_edit_name.clone().unwrap() && nodes_lock.iter().any(|n| n.name == node.name) {
                ui.colored_label(Color32::RED, "Oprez! Već postoji čvor s istim nazivom.");
            }
            else {
                ui.add_space(17.5);
            }
            drop(nodes_lock);

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    if !id_exists {
                        node.center = (node.center - app.mouse_drag_delta) / app.zoom;   // kompenzacija za zoom i drag
                        
                        let node_to_edit_id = app.node_to_edit_id.clone().unwrap();
                        let mut nodes = app.nodes_arc.lock().unwrap();
                        for n in &mut *nodes {
                            if n.id == node_to_edit_id {
                                *n = node.clone();
                                break;
                            }
                        }
                        if node.id != node_to_edit_id {
                            let mut links = app.links_arc.lock().unwrap();
                            links.iter_mut().for_each(|l| {
                                if l.node1_id == node_to_edit_id {
                                    l.node1_id = node.id.clone();
                                }
                                if l.node2_id == node_to_edit_id {
                                    l.node2_id = node.id.clone();
                                }
                            })
                        }
                    }
                    app.node_editing = false;
                    app.node_to_edit = None;
                    app.node_to_edit_id = None;
                    app.node_to_edit_name = None;
                }

                if ui.button("Odustani").clicked() {
                    app.node_editing = false;
                    app.node_to_edit = None;
                    app.node_to_edit_id = None;
                    app.node_to_edit_name = None;
                }
            });

            if app.node_editing {
                app.node_to_edit = Some(node);
            }
        });
}