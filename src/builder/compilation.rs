use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

fn is_rebuild_required(c_file: &Path, obj_file: &Path) -> bool {
    if !obj_file.exists() {
        return true;
    }
    let c_meta = fs::metadata(c_file).ok();
    let o_meta = fs::metadata(obj_file).ok();
    if let (Some(c_m), Some(o_m)) = (c_meta, o_meta) {
        let c_time = c_m.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let o_time = o_m.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        return c_time > o_time;
    }
    true
}

pub fn compile_c_files(
    compiler: &str,
    c_files: &[PathBuf],
    obj_dir: &Path,
    include_dir: &str,
    c_flags: &str,
) -> Vec<PathBuf> {
    fs::create_dir_all(obj_dir).expect("Failed to create object directory");

    let pb = ProgressBar::new(c_files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=> "),
    );

    let mut objects = vec![];
    for c_file in c_files {
        let obj_path = obj_dir.join(
            c_file
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
                + ".o"
            );

        if is_rebuild_required(c_file, &obj_path) {
            pb.set_message(format!("Compiling {}", c_file.display()));

            let mut cmd = Command::new(compiler);
            cmd.arg("-I".to_owned() + include_dir);
            for flag in c_flags.split(' ') {
                cmd.arg(flag);
            }
            cmd.arg("-c").arg(c_file).arg("-o").arg(&obj_path);

            let status = cmd.status().expect("Failed to run compiler");

            if !status.success() {
                pb.finish_and_clear();
                panic!("Compilation failed for {:?}", c_file);
            }
        } else {
            pb.set_message(format!("Cached    {}", c_file.display()));
        }

        objects.push(obj_path);
        pb.inc(1);
    }
    pb.finish_with_message("Compilation done");
    objects
}
