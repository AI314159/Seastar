use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

pub struct LanguageBuilder<'a> {
    pub name: &'a str,
    pub extensions: &'a [&'a str],
    pub compiler: &'a str,
    pub include_flag: Option<&'a str>, // e.g., "-I"
    pub compile_flags: &'a str,
}

fn is_rebuild_required(src_file: &Path, obj_file: &Path) -> bool {
    if !obj_file.exists() {
        return true;
    }
    let src_meta = fs::metadata(src_file).ok();
    let obj_meta = fs::metadata(obj_file).ok();
    if let (Some(s_m), Some(o_m)) = (src_meta, obj_meta) {
        let s_time = s_m.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let o_time = o_m.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        return s_time > o_time;
    }
    true
}

pub fn compile_files(
    lang: &LanguageBuilder,
    src_files: &[PathBuf],
    obj_dir: &Path,
    include_dir: Option<&str>,
) -> Vec<PathBuf> {
    fs::create_dir_all(obj_dir).expect("Failed to create object directory");

    let pb = ProgressBar::new(src_files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(&format!(
            "[{{elapsed_precise}}] [{{bar:40.cyan/blue}}] {{pos}}/{{len}} ({}: {{msg}})",
            lang.name
        ))
        .unwrap()
        .progress_chars("=> "),
    );

    let mut objects = vec![];
    for src_file in src_files {
        let stem = src_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let ext = src_file.extension().and_then(|s| s.to_str()).unwrap_or("");
        let obj_name = format!("{}.{}.o", stem, ext);
        let obj_path = obj_dir.join(obj_name);

        let matches_lang = src_file.extension().is_some_and(|e| {
            lang.extensions
                .iter()
                .any(|ext| e.eq_ignore_ascii_case(*ext))
        });
        if !matches_lang {
            continue;
        }

        if is_rebuild_required(src_file, &obj_path) {
            pb.set_message(format!("Compiling {}", src_file.display()));

            let mut cmd = Command::new(lang.compiler);

            if let (Some(flag), Some(inc)) = (lang.include_flag, include_dir) {
                cmd.arg(format!("{}{}", flag, inc));
            }

            for flag in lang.compile_flags.split_whitespace() {
                if !flag.is_empty() {
                    cmd.arg(flag);
                }
            }
            cmd.arg("-c").arg(src_file).arg("-o").arg(&obj_path);

            let status = cmd.status().expect("Failed to run compiler");

            if !status.success() {
                pb.finish_and_clear();
                panic!("Compilation failed for {:?}", src_file);
            }
        } else {
            pb.set_message(format!("Cached    {}", src_file.display()));
        }

        objects.push(obj_path);
        pb.inc(1);
    }
    pb.finish_with_message("Compilation done");
    objects
}
