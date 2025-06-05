use std::process::{Command, exit};

use crate::builder;
use crate::io;

pub fn run() {
    let config = io::load_config("Seastar.toml");
    let output_path = builder::build(&config, "src", "target/obj", "target");

    let status = Command::new(output_path)
        .status()
        .expect("Failed to run program. Maybe try running it manually?");
    if !status.success() {
        exit(status.code().unwrap_or(0));
    }
}

pub fn build() {
    let config = io::load_config("Seastar.toml");

    let output_path = builder::build(&config, "src", "target/obj", "target");
    println!("Successfully built to {}.", output_path);
}

pub fn clean() {
    let target_path = "target";
    let deps_path = "deps";

    if std::fs::exists("target").unwrap_or(false) {
        if std::fs::remove_dir_all(target_path).is_err() {
            println!("Failed to clean target directory: {}", target_path);
        }
    }
    if std::fs::exists("deps").unwrap_or(false) {
        if std::fs::remove_dir_all(deps_path).is_err() {
            println!("Failed to clean deps directory: {}", deps_path);
        }
    }
}
