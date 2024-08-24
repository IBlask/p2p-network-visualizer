use std::io::Write;
use crate::{Link, Node};
use std::fs::File;


pub fn save_to_graphml(nodes: &Vec<Node>, links: &Vec<Link>, file_path: &str) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;

    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<graphml xmlns="http://graphml.graphdrawing.org/xmlns">"#).map_err(|e| e.to_string())?;

    writeln!(file, r#"<key id="name" for="node" attr.name="name" attr.type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="pos_x" for="node" attr.name="pos_x" attr.type="integer"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="pos_y" for="node" attr.name="pos_y" attr.type="integer"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="ip_addr" for="node" attr.name="ip_addr" attr.type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="cpu" for="node" attr.name="cpu" attr.type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="ram" for="node" attr.name="ram" attr.type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="rom" for="node" attr.name="rom" attr.type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="os" for="node" attr.name="os" attr.type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="network_bw" for="node" attr.name="network_bw" attr.type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<key id="software" for="node" attr.name="software" attr.type="string"/>"#).map_err(|e| e.to_string())?;


    writeln!(file, r#"<graph id="G" edgedefault="undirected">"#).map_err(|e| e.to_string())?;

    for node in nodes {
        writeln!(file, r#"<node id="{}">"#, node.id).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="name">{}</data>"#, node.name).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="pos_x">{}</data>"#, node.center.x).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="pos_y">{}</data>"#, node.center.y).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="ip_addr">{}</data>"#, node.ip_addr).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="cpu">{}</data>"#, node.cpu).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="ram">{}</data>"#, node.ram).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="rom">{}</data>"#, node.rom).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="os">{}</data>"#, node.os).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="network_bw">{}</data>"#, node.network_bw).map_err(|e| e.to_string())?;
        writeln!(file, r#"<data key="software">{}</data>"#, node.software).map_err(|e| e.to_string())?;
        writeln!(file, r#"</node>"#).map_err(|e| e.to_string())?;
    }

    
    for link in links {
        writeln!(file, r#"<edge source="{}" target="{}"/>"#, link.node1_id, link.node2_id).map_err(|e| e.to_string())?;
    }

    
    writeln!(file, r#"</graph>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"</graphml>"#).map_err(|e| e.to_string())?;

    Ok(())
}