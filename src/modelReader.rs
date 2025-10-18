use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn read_obj_and_print(obj_path: &str) -> Result<()> {
    
    let obj_file = File::open(obj_path)?;
    let reader = BufReader::new(obj_file);

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("v ") { // Vértices
            let vertex_data: Vec<&str> = line.split_whitespace().collect();
            println!("Vértice: {}", vertex_data[1..].join(", "));
        } else if line.starts_with("vn ") { // Normales
            let normal_data: Vec<&str> = line.split_whitespace().collect();
            println!("Normal: {}", normal_data[1..].join(", "));
        } else if line.starts_with("usemtl ") { // Materiales
            let material_data: Vec<&str> = line.split_whitespace().collect();
            println!("Material: {}", material_data[1]);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let obj_path = "src/modelosOBJ/nave.obj";

    read_obj_and_print(obj_path)?;

    Ok(())
}