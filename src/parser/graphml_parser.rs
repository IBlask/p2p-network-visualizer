use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use crate::{Node, Link};

pub fn parse_graphml(nodes_arc: &mut Vec<Node>, links_arc: &mut Vec<Link>, reader: BufReader<File>) {
    let parser = EventReader::new(reader);

    nodes_arc.clear();
    nodes_arc.shrink_to_fit();
    links_arc.clear();
    links_arc.shrink_to_fit();

    let mut current_node_id = String::new();
    let mut current_node_name = String::new();
    let mut current_node_pos_x = 0.0;
    let mut current_node_pos_y = 0.0;
    let mut current_node_key_type = String::new();
    let mut current_source = String::new();
    let mut current_target = String::new();
    let mut id_map = HashMap::new();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "node" => {
                        for attr in attributes {
                            if attr.name.local_name == "id" {
                                current_node_id = attr.value.clone();
                            }
                        }
                    }
                    "data" => {
                        for attr in attributes {
                            if attr.name.local_name == "key" {
                                current_node_key_type = attr.value;
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
                        let link_data = Link {
                            node1_index: *id_map.get(&current_source).unwrap_or(&0),
                            node2_index: *id_map.get(&current_target).unwrap_or(&0),
                        };
                        links_arc.push(link_data);
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                if current_node_key_type == "name" {
                    current_node_name = text.parse::<String>().unwrap();
                } else if current_node_key_type == "pos_x" {
                    current_node_pos_x = text.parse::<f32>().unwrap();
                } else if current_node_key_type == "pos_y" {
                    current_node_pos_y = text.parse::<f32>().unwrap();
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "node" {
                    let node_data = Node {
                        id: current_node_id.clone(),
                        name: current_node_name.clone(),
                        center: egui::pos2(current_node_pos_x, current_node_pos_y),
                        radius: 15.0,
                        color: egui::Color32::WHITE,
                    };
                    nodes_arc.push(node_data);
                    id_map.insert(current_node_id.clone(), nodes_arc.len() - 1);
                }
            }
            _ => {}
        }
    }
}
