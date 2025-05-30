use serde::Deserialize;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project_name: String,
    pub compiler: String,
    pub options: Options,
}

#[derive(Debug, Deserialize, Default)]
pub struct Options {
    #[serde(default)]
    pub c_flags: String,
    #[serde(default)]
    pub link_flags: String,
}

pub fn load_config<P: Into<PathBuf>>(path: P) -> Config {
    let data = fs::read_to_string(path.into()).expect("Failed to read config");
    toml::from_str(&data).expect("Failed to parse TOML")
}

pub fn get_c_files(src_dir: &str) -> Vec<PathBuf> {
    WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "c"))
        .map(|e| e.path().to_path_buf())
        .collect()
}
