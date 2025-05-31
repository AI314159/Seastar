mod compilation;
mod linking;

use crate::io;
use std::path::PathBuf;

use compilation::LanguageBuilder;

pub fn build(config: &io::Config, source_dir: &str, obj_dir: &str, output_dir: &str) -> String {
    let supported_extensions = &["c", "cpp", "cc", "cxx", "c++"];
    let all_files = io::get_source_files(source_dir, supported_extensions);

    let languages = [
        LanguageBuilder {
            name: "C",
            extensions: &["c"],
            compiler: &config.compiler,
            include_flag: Some("-I"),
            compile_flags: &config.options.c_flags,
        },
        LanguageBuilder {
            name: "C++",
            extensions: &["cpp", "cc", "cxx", "c++"],
            compiler: config.cpp_compiler.as_deref().unwrap_or("g++"),
            include_flag: Some("-I"),
            compile_flags: config.options.cpp_flags.as_deref().unwrap_or(""),
        },
    ];

    let obj_dir = PathBuf::from(obj_dir);
    let include_dir = "include";

    let mut all_objects = Vec::new();

    for lang in &languages {
        let src_files: Vec<_> = all_files
            .iter()
            .filter(|f| {
                f.extension()
                    .and_then(|e| e.to_str())
                    .map(|ext| lang.extensions.iter().any(|&x| ext.eq_ignore_ascii_case(x)))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if !src_files.is_empty() {
            let objects = compilation::compile_files(lang, &src_files, &obj_dir, Some(include_dir));
            all_objects.extend(objects);
        }
    }

    let is_any_cpp = all_files.iter().any(|f| {
        f.extension()
            .and_then(|e| e.to_str())
            .map(|ext| ["cpp", "cc", "cxx", "c++"].contains(&ext))
            .unwrap_or(false)
    });
    let linker = if is_any_cpp {
        config.cpp_compiler.as_deref().unwrap_or("g++")
    } else {
        &config.compiler
    };
    let link_flags = if is_any_cpp {
        config.options.cpp_link_flags.as_deref().unwrap_or("")
    } else {
        &config.options.link_flags
    };

    let output_path = PathBuf::from(output_dir).join(&config.project_name);
    linking::link_objects(linker, &all_objects, &output_path, link_flags);

    println!();

    output_path.display().to_string()
}
