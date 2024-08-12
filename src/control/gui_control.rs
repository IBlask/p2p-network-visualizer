use eframe::egui;
use egui::Vec2;

use crate::MyApp;

pub fn setup_side_panel(ctx: &egui::Context, app: &mut MyApp) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.add(egui::Button::new("Dodaj čvor"));
        ui.add(egui::Button::new("Izbriši čvor"));
        ui.add(egui::Button::new("Dodaj vezu"));
        ui.add(egui::Button::new("Izbriši vezu"));
        ui.add(egui::Button::new("Spremi kao datoteku"));

        if ui.button("Učitaj iz datoteke").clicked() {
            if let Some(path) = rfd::FileDialog::new()
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

        let nodes_lock = app.nodes_arc.lock().unwrap();
        let links_lock = app.links_arc.lock().unwrap();

        let mut node_popup_name = String::new();
        let mut mouse_pos = ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default();
        let mut is_hovered: bool;

        for link in &*links_lock {
            painter.line_segment(
                [nodes_lock.get(link.node1_index).unwrap().center, nodes_lock.get(link.node2_index).unwrap().center],
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );
        }

        for node in &*nodes_lock {
            painter.circle_filled(node.center, node.radius, node.color);

            mouse_pos = ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default();
            is_hovered = (mouse_pos - node.center).length() <= node.radius;

            if is_hovered {
                node_popup_name = node.name.clone();
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
