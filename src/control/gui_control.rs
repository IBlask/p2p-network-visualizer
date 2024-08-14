use eframe::egui;
use egui::{Color32, Pos2, Vec2};

use crate::MyApp;

pub fn setup_side_panel(ctx: &egui::Context, app: &mut MyApp) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        if ui.button("Dodaj čvor").clicked() {
            app.adding_node = true;
        }
        if ui.button("Izbriši čvor").clicked() {
            app.deleting_node = true;
        }
        ui.add(egui::Button::new("Dodaj vezu"));
        ui.add(egui::Button::new("Izbriši vezu"));
        ui.add(egui::Button::new("Spremi kao datoteku"));

        if ui.button("Učitaj iz datoteke").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Učitaj iz datoteke")
                .add_filter("GraphML & GEXF", &["graphml", "gexf"])
                .add_filter("GraphML", &["graphml"])
                .add_filter("GEXF", &["gexf"])
                .pick_file() {
                    let path = Some(path.display().to_string());
                    let graphml_file = std::fs::File::open(path.unwrap()).expect("Otvori GraphML datoteku");
                    let reader = std::io::BufReader::new(graphml_file);
                    crate::parser::graphml_parser::parse_graphml(
                        &mut app.nodes_arc.lock().unwrap(),
                        &mut app.links_arc.lock().unwrap(),
                        reader,
                    );
                }
        }
    });
}

pub fn render_graph(ctx: &egui::Context, app: &mut MyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let painter = ui.painter();
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default();
        let mut node_popup_name = String::new();

        // Kopiranje čvorova i minimiziranje zaključavanja 
        let nodes_lock = app.nodes_arc.lock().unwrap();
        let nodes: Vec<_> = nodes_lock.clone();
        drop(nodes_lock);

        // Kopiranje veza i minimiziranje zaključavanja 
        let links_lock = app.links_arc.lock().unwrap();
        let links: Vec<_> = links_lock.clone();
        drop(links_lock);

        // Crtanje veza
        for link in &*links {
            let node1_pos = nodes.iter().position(|n| n.id == link.node1_id);
            let node2_pos = nodes.iter().position(|n| n.id == link.node2_id);

            if node1_pos.is_some() && node2_pos.is_some() {
                painter.line_segment(
                    [
                        nodes.get(node1_pos.unwrap()).unwrap().center,
                        nodes.get(node2_pos.unwrap()).unwrap().center
                    ],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                );
            }
        }

        // Crtanje čvorova
        for node in &*nodes {
            painter.circle_filled(node.center, node.radius, node.color);

            if (mouse_pos - node.center).length() <= node.radius {
                node_popup_name = node.name.clone();

                // Odabir kod brisanja čvora
                if app.deleting_node {
                    if app.left_click_released {
                        // označavanje crvenom bojom na prijelaz mišem
                        painter.circle_filled(node.center, node.radius, Color32::RED);
                    }
                    if ctx.input(|i| i.pointer.primary_clicked()) {
                        app.node_to_delete = Some(node.clone());
                        app.left_click_released = false;
                        app.show_delete_dialog = true;
                    }
                    if ctx.input(|i| i.pointer.primary_released()) {
                        app.left_click_released = true;
                        app.deleting_node = false;
                    }
                }
            }
        }


        // Dodavanje čvora
        if app.adding_node {
            painter.circle_filled(mouse_pos, 15.0, Color32::LIGHT_BLUE); 
            if ctx.input(|i| i.pointer.primary_clicked()) {
                app.new_node_pos = mouse_pos;
                app.show_input_dialog = true;
                app.adding_node = false; 
            }
        }

        // Unos podataka o novom čvoru
        if app.show_input_dialog {
            painter.circle_filled(app.new_node_pos, 15.0, Color32::LIGHT_BLUE);

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
                                    radius: 15.0,
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


        // Brisanje čvora
        if app.show_delete_dialog {
            let node_to_delete = app.node_to_delete.clone().unwrap();
            ui.painter().circle_filled(node_to_delete.center, node_to_delete.radius, Color32::RED);

            if app.left_click_released {
                // lijeva tipka miša više nije stisnuta

                egui::Window::new("Potvrda brisanja čvora")
                .collapsible(true)
                .resizable(false)
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


        show_popup(ui, ctx, mouse_pos, &node_popup_name);
    });
}

fn show_popup(ui: &mut egui::Ui, ctx: &egui::Context, pos: egui::Pos2, text: &str) {
    let painter = ui.painter();

    let font_id = egui::FontId::proportional(16.0);
    let text_size = ctx.fonts(|f| f.layout_no_wrap(text.to_string(), font_id.clone(), egui::Color32::BLACK)).size();

    let popup_pos = pos + Vec2::new(10.0, 10.0);

    painter.rect_filled(
        egui::Rect::from_min_size(popup_pos, text_size),
        4.0,
        egui::Color32::from_white_alpha(200),
    );
    painter.text(
        popup_pos,
        egui::Align2::LEFT_TOP,
        text,
        font_id,
        egui::Color32::BLACK,
    );
}
