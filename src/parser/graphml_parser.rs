use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use crate::{Link, MyApp, Node};

fn scale_float_to_int(value: f32) -> i32 {
    (value * 1000.0).round() as i32
}

pub fn parse_graphml(app: &MyApp, nodes_arc: &mut Vec<Node>, links_arc: &mut Vec<Link>, reader: BufReader<File>) {
    let parser = EventReader::new(reader);

    nodes_arc.clear();
    nodes_arc.shrink_to_fit();
    links_arc.clear();
    links_arc.shrink_to_fit();

    let mut new_node = Node::new();
    let mut current_key_type = String::new();
    let mut current_source = String::new();
    let mut current_target = String::new();
    let mut id_map = HashMap::new();
    let mut edges: Vec<(String, String)> = Vec::new();
    let mut nodes_without_pos: Vec<String> = Vec::new();
    let mut occupied_positions: HashSet<(i32, i32)> = HashSet::new();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "node" => {
                        new_node = Node::new();
                        for attr in attributes {
                            if attr.name.local_name == "id" {
                                new_node.id = attr.value.clone();
                            }
                        }
                    }
                    "data" => {
                        for attr in attributes {
                            if attr.name.local_name == "key" {
                                current_key_type = attr.value.clone();
                            }
                        }
                    }
                    "edge" => {
                        for attr in attributes {
                            match attr.name.local_name.as_str() {
                                "source" => current_source = attr.value.clone(),
                                "target" => current_target = attr.value.clone(),
                                _ => {}
                            }
                        }
                        edges.push((current_source.clone(), current_target.clone()));
                        let link_data = Link {
                            node1_id: current_source.clone(),
                            node2_id: current_target.clone(),
                        };
                        links_arc.push(link_data);
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                match current_key_type.as_str() {
                    "name" => new_node.name = text,
                    "pos_x" => new_node.center.x = text.parse::<f32>().unwrap(),
                    "pos_y" => new_node.center.y = text.parse::<f32>().unwrap(),
                    "ip_addr" => new_node.ip_addr = text,
                    "cpu" => new_node.cpu = text,
                    "ram" => new_node.ram = text,
                    "rom" => new_node.rom = text,
                    "os" => new_node.os = text,
                    "network_bw" => new_node.network_bw = text,
                    "software" => new_node.software = text,
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "node" {
                    new_node.radius = app.node_default_radius;
                    new_node.color = egui::Color32::WHITE;

                    if new_node.center.x == 0.0 && new_node.center.y == 0.0 {
                        nodes_without_pos.push(new_node.id.clone());
                    } else {
                        occupied_positions.insert((scale_float_to_int(new_node.center.x), scale_float_to_int(new_node.center.y)));
                    }

                    nodes_arc.push(new_node.clone());
                    id_map.insert(new_node.id.clone(), nodes_arc.len() - 1);
                }
            }
            _ => {}
        }
    }


    
    // Čvorovi bez zadane pozicije
    let mut new_positions: Vec<(usize, f32, f32)> = Vec::new();

    for node_id in nodes_without_pos {
        if let Some(node_index) = id_map.get(&node_id) {
            let connected_nodes: Vec<_> = edges.iter()
                .filter(|(source, target)| source == &node_id || target == &node_id)
                .map(|(source, target)| {
                    if source == &node_id { target } else { source }
                })
                .collect();

            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut count = 0;

            for connected_node_id in connected_nodes {
                if let Some(connected_node_index) = id_map.get(connected_node_id) {
                    let connected_node = &nodes_arc[*connected_node_index];
                    if connected_node.center.x != 0.0 || connected_node.center.y != 0.0 {
                        sum_x += connected_node.center.x;
                        sum_y += connected_node.center.y;
                        count += 1;
                    }
                }
            }

            let mut pos_x;
            let pos_y;

            if count > 0 {
                pos_x = sum_x / count as f32;
                pos_y = sum_y / count as f32;
            } else {
                // ako svi susjedni čvorovi nemaju zadanu poziciju, postavi default: (400, 200)
                pos_x = 400.0;
                pos_y = 200.0;
            }

            // ako već postoji čvor na toj poziciji, pomakni novi desno
            while occupied_positions.contains(&(scale_float_to_int(pos_x), scale_float_to_int(pos_y))) {
                pos_x += 35.0;
            }
            occupied_positions.insert((scale_float_to_int(pos_x), scale_float_to_int(pos_y)));

            new_positions.push((*node_index, pos_x, pos_y));
        }
    }

    for (node_index, pos_x, pos_y) in new_positions {
        let node = &mut nodes_arc[node_index];
        node.center.x = pos_x;
        node.center.y = pos_y;
    }
}
