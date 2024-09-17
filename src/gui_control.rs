mod painter;
mod adding_node;
mod deleting_node;
mod adding_link;
mod deleting_link;
mod editing_node;


use crate::models::{MyApp, Node};

use eframe::egui;
use egui::{Button, Color32, Vec2};


pub fn setup_side_panel(ctx: &egui::Context, app: &mut MyApp) {
    egui::Area::new("side_panel")
        .order(egui::Order::Foreground)
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            egui::Frame::default()
                .fill(app.left_side_panel_color)
                .show(ui, |ui| {
                    ui.set_width(app.left_side_panel_width);
                    ui.set_height(ui.available_height());

                    let button_size = egui::vec2(ui.available_width(), 30.0);  // Full width, with a fixed height

                    if ui.add_sized(button_size, Button::new("Dodaj čvor")).clicked() {
                        app.adding_node = true;
                        app.new_node = Node::new();
                    }
                    if ui.add_sized(button_size, Button::new("Izbriši čvor")).clicked() {
                        app.deleting_node = true;
                    }

                    ui.add_space(20.0);

                    if ui.add_sized(button_size, Button::new("Dodaj vezu")).clicked() {
                        app.adding_link = true;
                    }
                    if ui.add_sized(button_size, Button::new("Izbriši vezu")).clicked() {
                        app.deleting_link = true;
                    }

                    ui.add_space(20.0);

                    if ui.add_sized(button_size, Button::new("Uredi detalje čvora")).clicked() {
                        app.node_editing = true;
                    }

                    ui.add_space(20.0);

                    if ui.add_sized(button_size, Button::new("Spremi kao datoteku")).clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("Spremi datoteku kao")
                            .add_filter("GraphML & GEXF", &["graphml", "gexf"])
                            .add_filter("GraphML", &["graphml"])
                            .add_filter("GEXF", &["gexf"])
                            .save_file() {
                            match crate::nff_utils::save_to_file(app, path) {
                                Ok(_) => {}
                                Err(error_message) => {
                                    app.show_error = true;
                                    app.error_message = error_message;
                                }
                            }
                        }
                    }
                    
                    if ui.add_sized(button_size, Button::new("Učitaj iz datoteke")).clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("Učitaj iz datoteke")
                            .add_filter("GraphML & GEXF", &["graphml", "gexf"])
                            .add_filter("GraphML", &["graphml"])
                            .add_filter("GEXF", &["gexf"])
                            .pick_file() {
                                let path_str = path.display().to_string();

                                let parse_result = {
                                    let mut nodes = app.nodes_arc.lock().unwrap();
                                    let mut links = app.links_arc.lock().unwrap();

                                    crate::nff_utils::parse_file(app, &mut nodes, &mut links, &path_str)
                                };

                                match parse_result {
                                    Ok(_success) => {
                                        let mut nodes = app.nodes_arc.lock().unwrap();
                                        if nodes.iter().any(|n| n.center.x < app.left_side_panel_width) {
                                            for n in nodes.iter_mut() {
                                                n.center.x += app.left_side_panel_width;
                                            }
                                        }
                                        
                                        app.zoom = 1.0;
                                        app.mouse_drag_delta = Vec2::default();
                                    }
                                    Err(error_message) => {
                                        app.show_error = true;
                                        app.error_message = error_message;
                                    }
                                }
                        }
                    }

                    ui.add_space(40.0);

                    ui.checkbox(&mut app.show_node_names, "Prikaži nazive čvorova");

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        let default_node_color = app.default_node_color;
                        let button_text = if default_node_color == Color32::WHITE {
                            "Svijetla tema"
                        } else {
                            "Tamna tema"
                        };

                        if ui.add_sized(button_size, Button::new(button_text)).clicked() {
                            if default_node_color == Color32::WHITE {
                                app.set_light_theme();
                            }
                            else {
                                app.set_dark_theme();
                            }
                        }
                    });
                
                });
        });
}



pub fn render_graph(ctx: &egui::Context, app: &mut MyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {

        // ako je došlo do greške na parseru - pokaži popup
        if app.show_error {
            egui::Window::new("Greška")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.colored_label(
                    Color32::RED, 
                    app.error_message.clone());
                if ui.button("OK").clicked() {
                    app.show_error = false;
                }
            });
            return;
        }

        // Očitanje pozicije miša
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default();

        // Pomicanje mreže mišem
        if ctx.input(|i| i.pointer.middle_down()) {
            if app.dragging {
                app.mouse_drag_delta += ctx.input(|i| i.pointer.delta());
            } else {
                app.dragging = true;
            }
        }
        if ctx.input(|i| i.pointer.any_released()) {
            app.dragging = false;           // pomicanje cijele mreže
            app.dragged_node_id = None;     // pomicanje pojedinog čvora
        }

        // Zoom
        let scroll_delta = ctx.input(|i| i.scroll_delta.y);
        if scroll_delta != 0.0 {
            app.zoom *= 1.0 + scroll_delta * 0.001;
        }

        // Kopiranje čvorova 
        let mut nodes_lock = app.nodes_arc.lock().unwrap();
        let mut nodes: Vec<_> = nodes_lock.clone();
        // Pomicanje pojedinih čvorova
        if let Some(dragged_node_id) = &app.dragged_node_id {
            if let Some(node) = nodes_lock.iter_mut().find(|node| &node.id == dragged_node_id) {
                let delta = ctx.input(|i| i.pointer.delta());
                node.center = node.center + delta / app.zoom;
            }
        }
        // Minimiziranje zaključavanja
        drop(nodes_lock);

        // Kopiranje veza i minimiziranje zaključavanja 
        let links_lock = app.links_arc.lock().unwrap();
        let links: Vec<_> = links_lock.clone();
        drop(links_lock);


        // Crtanje veza
        painter::draw_links(ui, app, &nodes, &links);

        // Crtanje čvorova
        painter::draw_nodes(ui, ctx, app, mouse_pos, &mut nodes);


        // Dodavanje čvora
        if app.adding_node {
            adding_node::adding_node(ui, ctx, app, mouse_pos);
        }

        // Unos podataka o novom čvoru
        if app.show_input_dialog {
            adding_node::show_input_dialog(ui, ctx, app, &nodes);
        }


        // Brisanje čvora
        if app.show_delete_dialog {
            deleting_node::show_delete_dialog(ui, ctx, app);
        }


        // Dodavanje veze
        if app.adding_link {
            adding_link::adding_link(ui, ctx, app);
        }

        // Brisanje veze
        if app.deleting_link {
            deleting_link::deleting_link(ui, ctx, app);
        }


        // Uređivanje atributa čvora
        if app.node_editing && app.node_to_edit.is_some() {
            editing_node::show_node_editing_dialog(ui, ctx, app);
        }


        // Popup s detaljima čvora
        if app.node_popup.is_some() {
            painter::show_popup(&ui, ctx, mouse_pos, app);
            app.node_popup = None;
        }
    });
}
