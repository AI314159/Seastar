use std::collections::VecDeque;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const TEMPLATE_PATH: &str = "~/.seastar/templates";
const TEMPLATE_REPO: &str = "https://github.com/AI314159/seastar-templates.git";

fn get_template_name(language: &str, is_lib: bool) -> String {
    let prefix = if is_lib { "lib" } else { "bin" };
    format!("{}-{}", language, prefix)
}

pub fn template(
    language: &str,
    is_lib: bool,
    copy_to: &PathBuf,
    compiler: &str,
    cpp_compiler: &str,
    project_name: &str,
) {
    clone_templates(TEMPLATE_REPO);

    if copy_to.exists() {
        eprintln!(
            "{:?} already exists, failed to initialize template.",
            copy_to
        );
        return;
    }

    copy_template(
        language,
        is_lib,
        copy_to,
        compiler,
        cpp_compiler,
        project_name,
    );

    eprintln!(
        "Initialized {} package '{}' in current working directory!",
        if is_lib {
            "library"
        } else {
            "binary"
        },
        project_name
    );
}

fn copy_template(
    language: &str,
    is_lib: bool,
    copy_to: &PathBuf,
    compiler: &str,
    cpp_compiler: &str,
    project_name: &str,
) {
    // Expand the template path first
    let expanded_template_dir = shellexpand::tilde(TEMPLATE_PATH).to_string();
    let folder_name = get_template_name(language, is_lib);
    let template_path = Path::new(&expanded_template_dir).join(folder_name);

    if !template_path.exists() || !template_path.is_dir() {
        panic!(
            "Template path does not exist or is not a directory: {:?}",
            template_path
        );
    }

    let mut queue = VecDeque::new();
    queue.push_back((template_path.to_path_buf(), copy_to.to_path_buf()));

    while let Some((src, dst)) = queue.pop_front() {
        if src.is_dir() {
            fs::create_dir_all(&dst).expect("Failed to create destination directory");

            for entry in fs::read_dir(&src).expect("Failed to read template directory") {
                let entry = entry.expect("Failed to read entry");
                let entry_path = entry.path();
                let file_name = entry.file_name();
                let dst_path = dst.join(file_name);
                queue.push_back((entry_path, dst_path));
            }
        } else if src.is_file() {
            let mut content = String::new();
            fs::File::open(&src)
                .expect("Failed to open template file")
                .read_to_string(&mut content)
                .expect("Failed to read template file");

            let content = content
                .replace("{{compiler}}", compiler)
                .replace("{{cpp_compiler}}", cpp_compiler)
                .replace("{{project_name}}", project_name);

            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directory");
            }

            let mut dest_file = fs::File::create(&dst).expect("Failed to create destination file");
            dest_file
                .write_all(content.as_bytes())
                .expect("Failed to write to destination file");
        }
    }
}

fn clone_templates(repo: &str) {
    let expanded_template_dir = shellexpand::tilde(TEMPLATE_PATH).to_string();
    if !Path::new(&expanded_template_dir).exists() {
        fs::create_dir_all(&expanded_template_dir).expect("Failed to create template directory");

        eprintln!("Templates not found, cloning into {}", TEMPLATE_PATH);

        let output = Command::new("git")
            .args(&["clone", "--depth", "1", repo, &expanded_template_dir])
            .output()
            .expect("Failed to execute git clone. Are you sure git is installed?");

        if !output.status.success() {
            eprintln!(
                "Error cloning template repository: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        eprintln!("Finished clone!");
    }
}
