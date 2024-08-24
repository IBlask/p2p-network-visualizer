use std::io::Write;
use crate::{Link, Node};
use std::fs::File;


pub fn save_to_gexf(nodes: &Vec<Node>, links: &Vec<Link>, file_path: &str) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;

    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<gexf xmlns="http://gexf.net/1.2" version="1.2">"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<graph mode="static" defaultedgetype="undirected">"#).map_err(|e| e.to_string())?;

    writeln!(file, r#"<attributes class="node">"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="pos_x" title="pos_x" type="integer"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="pos_y" title="pos_y" type="integer"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="ip_addr" title="ip_addr" type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="cpu" title="cpu" type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="ram" title="ram" type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="rom" title="rom" type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="os" title="os" type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="network_bw" title="network_bw" type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"<attribute id="software" title="software" type="string"/>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"</attributes>"#).map_err(|e| e.to_string())?;

    
    writeln!(file, r#"<nodes>"#).map_err(|e| e.to_string())?;
    for node in nodes {
        writeln!(file, r#"<node id="{}" label="{}">"#, node.id, node.name).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalues>"#).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="pos_x" value="{}"/>"#, node.center.x).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="pos_y" value="{}"/>"#, node.center.y).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="ip_addr" value="{}"/>"#, node.ip_addr).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="cpu" value="{}"/>"#, node.cpu).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="ram" value="{}"/>"#, node.ram).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="rom" value="{}"/>"#, node.rom).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="os" value="{}"/>"#, node.os).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="network_bw" value="{}"/>"#, node.network_bw).map_err(|e| e.to_string())?;
        writeln!(file, r#"<attvalue for="software" value="{}"/>"#, node.software).map_err(|e| e.to_string())?;
        writeln!(file, r#"</attvalues>"#).map_err(|e| e.to_string())?;
        writeln!(file, r#"</node>"#).map_err(|e| e.to_string())?;
    }
    writeln!(file, r#"</nodes>"#).map_err(|e| e.to_string())?;

    
    writeln!(file, r#"<edges>"#).map_err(|e| e.to_string())?;
    for link in links {
        writeln!(file, r#"<edge id="{}" source="{}" target="{}"/>"#, link.node1_id, link.node1_id, link.node2_id).map_err(|e| e.to_string())?;
    }
    writeln!(file, r#"</edges>"#).map_err(|e| e.to_string())?;

    
    writeln!(file, r#"</graph>"#).map_err(|e| e.to_string())?;
    writeln!(file, r#"</gexf>"#).map_err(|e| e.to_string())?;

    Ok(())
}