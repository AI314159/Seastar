use indicatif::ProgressBar;
use std::{path::PathBuf, process::Command, time::Duration};

pub fn link_objects(
    compiler: &str,
    objects: &[PathBuf],
    output: &PathBuf,
    is_library: &bool,
    link_flags: &str,
) {
    let pb = ProgressBar::new_spinner();
    pb.set_message("Linking...");
    pb.enable_steady_tick(Duration::new(0, 50_000_000));

    if *is_library {
        let mut cmd = Command::new("ar"); // TODO: configurable
        cmd.arg("rcs");
        cmd.arg(output);

        for obj in objects {
            cmd.arg(obj);
        }

        let status = cmd.status().expect("Failed to run linker");
        pb.finish_with_message("Static library created");

        if !status.success() {
            panic!("Static linking failed");
        }
    } else {
        let mut cmd = Command::new(compiler);
        let mut did_see_lib = false;
        // Insert start-group before first static library, and end-group after.
        // Not really sure why this is needed, but it removed the link errors.
        // TODO: instead of doing it this way, we should pass libraries as a
        // separate argument for clarity.
        let mut args = Vec::new();
        for obj in objects {
            let is_a = obj.extension().map(|e| e == "a").unwrap_or(false);
            if is_a && !did_see_lib {
                args.push("-Wl,--start-group".into());
                did_see_lib = true;
            }
            args.push(obj.to_string_lossy().into_owned());
        }
        if did_see_lib {
            args.push("-Wl,--end-group".into());
        }
        for arg in &args {
            cmd.arg(arg);
        }
        for flag in link_flags.split_whitespace() {
            if !flag.is_empty() {
                cmd.arg(flag);
            }
        }
        cmd.arg("-o").arg(output);
        println!("COMMAND: {:?}", cmd);
        let status = cmd.status().expect("Failed to run linker");
        pb.finish_with_message("Linking complete");

        if !status.success() {
            panic!("Linking failed");
        }
    }
}
