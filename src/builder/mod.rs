mod compilation;
mod linking;

use crate::{
    depman::{
        self,
        resolver::{Dep, parse_deps, resolve_and_fetch},
    },
    io,
};
use std::path::PathBuf;

use compilation::LanguageBuilder;

fn build_deps(deps: &[Dep]) -> Vec<PathBuf> {
    let graph = resolve_and_fetch(&deps, "deps/");

    let mut dep_static_libs = Vec::new();
    for dep_node in graph.topological_order() {
        let dep_name = &dep_node.dep.name;
        let dep_path = format!("deps/{}", dep_name);

        let src_headers = PathBuf::from(&dep_path).join("external_headers");
        let dst_headers = PathBuf::from("deps").join("headers").join(dep_name);
        if src_headers.exists() {
            println!(
                "Copying headers from {:?} to {:?}",
                src_headers, dst_headers
            );
            depman::fs_copy::copy_dir_recursive(&src_headers, &dst_headers)
                .expect("Failed to copy headers");
        }

        let dep_src = PathBuf::from(&dep_path).join("src");
        let dep_obj_dir = PathBuf::from(&dep_path).join("obj");
        let dep_lib = PathBuf::from(&dep_path).join(format!("lib{}.a", dep_name));

        let dep_src_files =
            io::get_source_files(dep_src.to_str().unwrap(), &["c", "cpp", "cc", "cxx", "c++"]);

        if !dep_src_files.is_empty() {
            let mut objects = Vec::new();

            let c_files: Vec<_> = dep_src_files
                .iter()
                .filter(|f| {
                    f.extension()
                        .and_then(|e| e.to_str())
                        .map(|ext| ext == "c")
                        .unwrap_or(false)
                })
                .cloned()
                .collect();

            if !c_files.is_empty() {
                objects.extend(compilation::compile_files(
                    &LanguageBuilder {
                        name: "C",
                        extensions: &["c"],
                        compiler: "gcc",
                        include_flag: Some("-I"),
                        compile_flags: "",
                    },
                    &c_files,
                    &dep_obj_dir,
                    &[PathBuf::from("deps/headers")],
                ));
            }

            let cpp_files: Vec<_> = dep_src_files
                .iter()
                .filter(|f| {
                    f.extension()
                        .and_then(|e| e.to_str())
                        .map(|ext| ["cpp", "cc", "cxx", "c++"].contains(&ext))
                        .unwrap_or(false)
                })
                .cloned()
                .collect();

            if !cpp_files.is_empty() {
                objects.extend(compilation::compile_files(
                    &LanguageBuilder {
                        name: "C++",
                        extensions: &["cpp", "cc", "cxx", "c++"],
                        compiler: "g++",
                        include_flag: Some("-I"),
                        compile_flags: "",
                    },
                    &cpp_files,
                    &dep_obj_dir,
                    &[PathBuf::from("deps/headers")],
                ));
            }

            linking::link_objects("ar", &objects, &dep_lib, &true, "");
            dep_static_libs.push(dep_lib);
        }
    }
    dep_static_libs
}

pub fn build(config: &io::Config, source_dir: &str, obj_dir: &str, output_dir: &str) -> String {
    let deps = parse_deps(&config);
    let dep_statics = build_deps(&deps);

    let supported_extensions = &["c", "cpp", "cc", "cxx", "c++"];
    let all_files = io::get_source_files(source_dir, supported_extensions);

    let languages = [
        LanguageBuilder {
            name: "C",
            extensions: &["c"],
            compiler: &config.package.compiler,
            include_flag: Some("-I"),
            compile_flags: &config.options.c_flags,
        },
        LanguageBuilder {
            name: "C++",
            extensions: &["cpp", "cc", "cxx", "c++"],
            compiler: config.package.cpp_compiler.as_deref().unwrap_or("g++"),
            include_flag: Some("-I"),
            compile_flags: config.options.cpp_flags.as_deref().unwrap_or(""),
        },
    ];

    let obj_dir = PathBuf::from(obj_dir);

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
        let include_dirs = vec![
            PathBuf::from("deps").join("headers"),
            PathBuf::from("include"),
        ];

        if !src_files.is_empty() {
            let objects = compilation::compile_files(lang, &src_files, &obj_dir, &include_dirs);
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
        config.package.cpp_compiler.as_deref().unwrap_or("g++")
    } else {
        &config.package.compiler
    };
    let link_flags = if is_any_cpp {
        config.options.cpp_link_flags.as_deref().unwrap_or("")
    } else {
        &config.options.link_flags
    };

    let output_path = if config.package.is_library {
        PathBuf::from(output_dir)
            .join(&config.package.project_name)
            .with_extension("a")
    } else {
        PathBuf::from(output_dir).join(&config.package.project_name)
    };

    all_objects.extend_from_slice(&dep_statics);
    linking::link_objects(
        linker,
        &all_objects,
        &output_path,
        &config.package.is_library,
        link_flags,
    );

    println!();

    output_path.display().to_string()
}
