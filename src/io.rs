use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};


#[derive(Debug, Deserialize)]
pub struct Config {
    pub project_name: String,
    pub compiler: String,

    #[serde(default)]
    pub is_library: bool,

    #[serde(default)]
    pub cpp_compiler: Option<String>,
    pub options: Options,

    #[serde(default)]
    pub dependencies: HashMap<String, DepSpec>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Options {
    #[serde(default)]
    pub c_flags: String,
    #[serde(default)]
    pub link_flags: String,

    #[serde(default)]
    pub cpp_flags: Option<String>,
    #[serde(default)]
    pub cpp_link_flags: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum DepSpec {
    Simple(String),
    Detailed {
        git: Option<String>,
        tag: Option<String>,
        path: Option<String>,
    }
}

pub fn load_config<P: Into<PathBuf>>(path: P) -> Config {
    let data = fs::read_to_string(path.into()).expect("Failed to read config");
    toml::from_str(&data).expect("Failed to parse TOML")
}

pub fn get_source_files(src_dir: &str, exts: &[&str]) -> Vec<PathBuf> {
    use walkdir::WalkDir;
    WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| exts.iter().any(|&e| e.eq_ignore_ascii_case(ext)))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}
