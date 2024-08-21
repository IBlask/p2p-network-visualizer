use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use crate::{Link, MyApp, Node};

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

                    nodes_arc.push(new_node.clone());
                    id_map.insert(new_node.id.clone(), nodes_arc.len() - 1);
                }
            }
            _ => {}
        }
    }
}
