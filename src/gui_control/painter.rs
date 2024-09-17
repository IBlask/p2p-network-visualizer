use crate::models::{Link, MyApp, Node};

use egui::{Color32, Pos2, Ui, Vec2};


pub fn draw_links(ui: &Ui, app: &MyApp, nodes: &Vec<Node>, links: &Vec<Link>) {
    for link in &*links {
        let mut tmp_vec = nodes.clone();
        tmp_vec.retain(|n| n.id == link.node1_id || n.id == link.node2_id);

        if tmp_vec.len() >= 2 {
            ui.painter().line_segment(
                [
                    scale_pos(tmp_vec.get(0).unwrap().center, app),
                    scale_pos(tmp_vec.get(1).unwrap().center, app),
                ],
                egui::Stroke::new(1.0, app.default_node_color),
            );
        }
    }
}


pub fn draw_nodes(ui: &Ui, ctx: &egui::Context, app: &mut MyApp, mouse_pos: Pos2, nodes: &mut Vec<Node>) {
    for node in &mut *nodes {
        node.center = scale_pos(node.center, app);

        if node.color == Color32::WHITE {
            node.color = app.default_node_color;
        }
        else {
            node.color = app.offline_node_color;
        }
        ui.painter().circle_filled(node.center, node.radius, node.color);

        // Prikazivanje naziva iznad čvora ako je checkbox označen
        if app.show_node_names {
            ui.painter().text(
                node.center + egui::Vec2::new(0.0, -node.radius - 8.0),
                egui::Align2::CENTER_CENTER,
                &node.name,
                egui::FontId::proportional(14.0),
                app.default_node_color,
            );
        }

        if (mouse_pos - node.center).length() <= node.radius {
            app.node_popup = Some(node.clone());

            // Pomicanje čvora
            if ctx.input(|i| i.pointer.primary_clicked()) && app.dragged_node_id.is_none() {
                app.dragged_node_id = Some(node.id.clone());
            }

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

            // Odabir kod dodavanje i brisanja veze
            if app.adding_link != app.deleting_link {
                ui.painter().circle_filled(node.center, node.radius, Color32::YELLOW);

                if ctx.input(|i| i.pointer.primary_clicked()) {
                    if app.first_node_selected.is_none() {
                        app.first_node_selected = Some(node.clone());
                    }
                    else {
                        app.second_node_selected = Some(node.clone());
                    }
                }
            }

            // Odabir kod uređivanja atributa čvora
            if app.node_editing && app.node_to_edit.is_none() {
                ui.painter().circle_filled(node.center, node.radius, Color32::BLUE);
                if ctx.input(|i| i.pointer.primary_clicked()) {
                    app.node_to_edit = Some(node.clone());
                }
            }
        }

        // konstantno prikazuj popup kod pomicanja čvora
        if app.dragged_node_id.is_some() && app.dragged_node_id.clone().unwrap() == node.id {
            app.node_popup = Some(node.clone());
        }
    }
}


pub fn show_popup(ui: &egui::Ui, ctx: &egui::Context, pos: egui::Pos2, app: &MyApp) {
    let painter = ui.painter();
    let node = app.node_popup.clone().unwrap();

    let text =
        node.name + "\n\n"
        + "ID: " + &node.id + "\n"
        + "IP adresa: " + &node.ip_addr + "\n"
        + "CPU: " + &node.cpu + "\n"
        + "RAM: " + &node.ram + "\n"
        + "ROM: " + &node.rom + "\n"
        + "Propusnost: " + &node.network_bw + "\n"
        + "OS: " + &node.os + "\n"
        + "Software: " + &node.software;

    let font_id = egui::FontId::proportional(16.0);
    let text_size = ctx.fonts(|f| f.layout_no_wrap(text.to_string(), font_id.clone(), egui::Color32::BLACK)).size();

    let margin = 6.0;
    let rect_size = text_size + Vec2::new(margin * 2.0, margin * 2.0);

    let popup_pos = pos + Vec2::new(10.0, 10.0);

    painter.rect_filled(
        egui::Rect::from_min_size(popup_pos, rect_size),
        4.0,
        egui::Color32::from_white_alpha(200),
    );
    painter.text(
        popup_pos + Vec2::new(margin, margin),
        egui::Align2::LEFT_TOP,
        text,
        font_id,
        egui::Color32::BLACK,
    );
}



pub fn scale_pos(pos: Pos2, app: &MyApp) -> Pos2 {
    Pos2 {
        x: pos.x * app.zoom + app.mouse_drag_delta.x,
        y: pos.y * app.zoom + app.mouse_drag_delta.y,
    }
}