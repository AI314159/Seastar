use std::{path::PathBuf, process::Command};

pub fn link_objects(compiler: &str, objects: &[PathBuf], output: &PathBuf) {
    let mut cmd = Command::new(compiler);
    for obj in objects {
        cmd.arg(obj);
    }
    cmd.arg("-o").arg(output);

    let status = cmd.status().expect("Failed to run linker");
    if !status.success() {
        panic!("Linking failed");
    }
}