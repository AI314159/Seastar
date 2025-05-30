use indicatif::ProgressBar;
use std::{path::PathBuf, process::Command, time::Duration};

pub fn link_objects(compiler: &str, objects: &[PathBuf], output: &PathBuf, link_flags: &str) {

    let pb = ProgressBar::new_spinner();
    pb.set_message("Linking...");
    pb.enable_steady_tick(Duration::new(0, 50_000_000));

    let mut cmd = Command::new(compiler);
    for obj in objects {
        cmd.arg(obj);
    }
    for flag in link_flags.split(' ') {
        cmd.arg(flag);
    }
    cmd.arg("-o").arg(output);

    let status = cmd.status().expect("Failed to run linker");
    pb.finish_with_message("Linking complete");

    if !status.success() {
        panic!("Linking failed");
    }
}
