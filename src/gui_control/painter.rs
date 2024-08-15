use crate::{Link, MyApp, Node};

use egui::{Color32, Pos2, Ui, Vec2};


pub fn draw_links(ui: &Ui, nodes: &Vec<Node>, links: &Vec<Link>) {
    for link in &*links {
        let mut tmp_vec = nodes.clone();
        tmp_vec.retain(|n| n.id == link.node1_id || n.id == link.node2_id);

        if tmp_vec.len() >= 2 {
            ui.painter().line_segment(
                [
                    tmp_vec.get(0).unwrap().center,
                    tmp_vec.get(1).unwrap().center,
                ],
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );
        }
    }
}


pub fn draw_nodes(ui: &Ui, ctx: &egui::Context, app: &mut MyApp, mouse_pos: Pos2, nodes: &Vec<Node>) {
    for node in &*nodes {
        ui.painter().circle_filled(node.center, node.radius, node.color);

        if (mouse_pos - node.center).length() <= node.radius {
            app.node_popup_name = Some(node.name.clone());

            // Odabir kod brisanja čvora
            if app.deleting_node {
                if app.left_click_released {
                    // označavanje crvenom bojom na prijelaz mišem
                    ui.painter().circle_filled(node.center, node.radius, Color32::RED);
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
}


pub fn show_popup(ui: &egui::Ui, ctx: &egui::Context, pos: egui::Pos2, text: String) {
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