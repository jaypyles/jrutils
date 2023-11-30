use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_dir_if_not_exists(dir: &Path) {
    let d = dir.clone();

    if !d.is_dir() {
        let created_dir = create_dir_all(d);
        match created_dir {
            Ok(val) => {}
            Err(error) => {
                println!("Error creating directory!");
            }
        }
    }
}

pub fn create_file_if_not_exists(file_path: &Path, contents: &str) {
    let f = file_path.clone();

    if !f.exists() {
        let mut toml = File::create(file_path).expect("Could not create file!");
        match toml.write_all(contents.as_bytes()) {
            Err(_) => {
                panic!("Error writing to config file.")
            }
            Ok(_) => {}
        }
    }
}
