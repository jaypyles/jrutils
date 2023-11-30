use std::fs;
use std::path::Path;
use toml::Value;

pub fn parse_toml(file_path: &Path) -> Value {
    // Read and parse a toml file
    let toml_content = fs::read_to_string(file_path).expect("Failed to read file!");
    let parsed_toml: Value = toml::from_str(&toml_content).expect("Failed to parse toml!");

    return parsed_toml;
}
