mod painter;
mod adding_node;
mod deleting_node;
mod adding_link;
mod deleting_link;


use crate::MyApp;

use eframe::egui;


pub fn setup_side_panel(ctx: &egui::Context, app: &mut MyApp) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        if ui.button("Dodaj čvor").clicked() {
            app.adding_node = true;
        }
        if ui.button("Izbriši čvor").clicked() {
            app.deleting_node = true;
        }
        if ui.button("Dodaj vezu").clicked() {
            app.adding_link = true;
        }
        if ui.button("Izbriši vezu").clicked() {
            app.deleting_link = true;
        }
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
                        app,
                        &mut app.nodes_arc.lock().unwrap(),
                        &mut app.links_arc.lock().unwrap(),
                        reader,
                    );
                }
        }

        ui.checkbox(&mut app.show_node_names, "Prikaži nazive čvorova");
    });
}


pub fn render_graph(ctx: &egui::Context, app: &mut MyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
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

        // Kopiranje čvorova 
        let mut nodes_lock = app.nodes_arc.lock().unwrap();
        let mut nodes: Vec<_> = nodes_lock.clone();
        // Pomicanje pojedinih čvorova
        if let Some(dragged_node_id) = &app.dragged_node_id {
            if let Some(node) = nodes_lock.iter_mut().find(|node| &node.id == dragged_node_id) {
                let delta = ctx.input(|i| i.pointer.delta());
                node.center += delta;
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


        // Popup s nazivom čvora
        if let Some(node_popup_name) = &app.node_popup_name {
            painter::show_popup(&ui, ctx, mouse_pos, node_popup_name.to_string());
            app.node_popup_name = None;
        }
    });
}
