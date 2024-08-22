mod graphml_parser;
mod gexf_parser;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::{Link, MyApp, Node};


pub fn parse_file(app: &MyApp, nodes_arc: &mut Vec<Node>, links_arc: &mut Vec<Link>, file_path: &str) -> Result<bool, String> {
    let path = Path::new(file_path);
    
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("graphml") => graphml_parser::parse_graphml(app, nodes_arc, links_arc, reader),
        Some("gexf") => gexf_parser::parse_gexf(app, nodes_arc, links_arc, reader),
        _ => Err("Nepodržani format datoteke! Učitajte isključivo .graphml ili .gexf datoteku.".to_string()),
    }
}
