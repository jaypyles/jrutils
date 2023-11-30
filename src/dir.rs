use std::fs::create_dir_all;
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
