use crate::io::{Config, Options};
use std::path::PathBuf;

// This trait is for any buildable language. Making it this way
// is a little annoying right now, but I think it will help
// later on when I am adding Rust support. We'll see.
pub trait Language {
    fn name(&self) -> &'static str;
    fn file_extensions(&self) -> &[&'static str];
    fn compiler(&self, config: &Config) -> String;
    fn compile_flags(&self, options: &Options) -> String;
    fn link_flags(&self, options: &Options) -> String;
    fn find_source_files(&self, src_dir: &str) -> Vec<PathBuf> {
        walkdir::WalkDir::new(src_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                let ext = e.path().extension().and_then(|e| e.to_str());
                self.file_extensions().iter().any(|&x| ext == Some(x))
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    }
}

pub struct CLang;
pub struct CppLang;

impl Language for CLang {
    fn name(&self) -> &'static str {
        "C"
    }

    fn file_extensions(&self) -> &[&'static str] {
        &["c"]
    }

    fn compiler(&self, config: &Config) -> String {
        config.compiler.clone()
    }
    fn compile_flags(&self, options: &Options) -> String {
        options.c_flags.clone()
    }
    fn link_flags(&self, options: &Options) -> String {
        options.link_flags.clone()
    }
}

impl Language for CppLang {
    fn name(&self) -> &'static str {
        "C++"
    }

    fn file_extensions(&self) -> &[&'static str] {
        // Why do people choose such weird names for C++ files; just use .cpp!
        &["cpp", "cc", "cxx", "c++"]
    }

    fn compiler(&self, config: &Config) -> String {
        config
            .cpp_compiler
            .clone()
            .unwrap_or("g++".to_string())
    }

    fn compile_flags(&self, options: &Options) -> String {
        options.cpp_flags.clone().unwrap_or_default()
    }
    fn link_flags(&self, options: &Options) -> String {
        options.cpp_link_flags.clone().unwrap_or_default()
    }
}
