use std::process::{exit, Command};

mod io;
mod builder;

fn main() {
    let config = io::load_config("Seastar.toml");


    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "build" {
            let output_path = builder::build(&config, "src", "target/obj", "target");
            println!("Successfully built to {}.", output_path);
        }
        if args[1] == "run" {
            let output_path = builder::build(&config, "src", "target/obj", "target");

            let status = Command::new(output_path)
                .status()
                .expect(
                    "Failed to run program. Maybe try running it manually?"
                );
            if !status.success() {
                exit(status.code().unwrap_or(0));
            }
        }
    }
}
