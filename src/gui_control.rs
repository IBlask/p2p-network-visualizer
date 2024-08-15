mod painter;
mod adding_node;
mod deleting_node;
mod adding_link;

use std::borrow::Borrow;

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
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default();

        // Kopiranje čvorova i minimiziranje zaključavanja 
        let nodes_lock = app.nodes_arc.lock().unwrap();
        let nodes: Vec<_> = nodes_lock.clone();
        drop(nodes_lock);

        // Kopiranje veza i minimiziranje zaključavanja 
        let links_lock = app.links_arc.lock().unwrap();
        let links: Vec<_> = links_lock.clone();
        drop(links_lock);

        // Crtanje veza
        painter::draw_links(ui.borrow(), &nodes, &links);

        // Crtanje čvorova
        painter::draw_nodes(ui.borrow(), ctx, app, mouse_pos, &nodes);


        // Dodavanje čvora
        if app.adding_node {
            adding_node::adding_node(ui.borrow(), ctx, app, mouse_pos);
        }

        // Unos podataka o novom čvoru
        if app.show_input_dialog {
            adding_node::show_input_dialog(ui.borrow(), ctx, app, &nodes);
        }


        // Brisanje čvora
        if app.show_delete_dialog {
            deleting_node::show_delete_dialog(ui.borrow(), ctx, app);
        }


        // Dodavanje veze
        if app.adding_link {
            adding_link::adding_link(ui.borrow(), ctx, app);
        }


        // Popup s nazivom čvora
        if let Some(node_popup_name) = &app.node_popup_name {
            painter::show_popup(&ui, ctx, mouse_pos, node_popup_name.to_string());
            app.node_popup_name = None;
        }
    });
}
