mod compilation;
mod linking;

use std::path::PathBuf;

use crate::io;

pub fn build(config: &io::Config, source_dir: &str, obj_dir: &str, output_dir: &str) -> String {
    let c_files = io::get_c_files(source_dir);

    let obj_dir = PathBuf::from(obj_dir);
    let objects = compilation::compile_c_files(
        &config.compiler, 
        &c_files, &obj_dir, 
        "include", 
        &config.options.c_flags
    );

    let output_path = PathBuf::from(output_dir).join(&config.project_name);
    linking::link_objects(&config.compiler, &objects, &output_path, &config.options.link_flags);
    
    println!();

    output_path.display().to_string()
}