mod graphml_parser;
mod gexf_parser;
mod graphml_saver;
mod gexf_saver;

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
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



pub fn save_to_file(app: &MyApp, path: PathBuf) -> Result<(), String> {
    let path_str = path.display().to_string();

    let provided_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let path = if provided_extension.is_empty() {
        if path.to_str().unwrap_or("").ends_with(".graphml") {
            path.with_extension("graphml")
        } else {
            path.with_extension("gexf")
        }
    } else {
        path
    };
    
    
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("graphml") => {
            if let Err(_error) = crate::nff_utils::graphml_saver::save_to_graphml(
                &app.nodes_arc.lock().unwrap(),
                &app.links_arc.lock().unwrap(),
                &path_str,
            ) {
                return Err("Greška prilikom spremanja. Pokušajte ponovno.".to_string());
            }
            Ok(())
        }
        Some("gexf") => {
            if let Err(_error) = crate::nff_utils::gexf_saver::save_to_gexf(
                &app.nodes_arc.lock().unwrap(),
                &app.links_arc.lock().unwrap(),
                &path_str,
            ) {
                return Err("Greška prilikom spremanja. Pokušajte ponovno.".to_string());
            }
            Ok(())
        }
        _ => {
            return Err("Nepodržani format. Odaberite isključivo .graphml ili .gexf format.".to_string());
        }
    }
}