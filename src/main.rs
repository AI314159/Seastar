use std::process::exit;

use clap::{Parser, Subcommand};

mod app;
mod builder;
mod depman;
mod io;
mod language;
mod templater;

#[derive(Parser)]
#[command(name = "seastar")]
#[command(about = "A project scaffolding tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the binary/static library
    Build,

    /// Build and run the binary
    Run,

    /// Clean compiled dependencies and object files
    Clean,

    /// Create a new project with a template
    New {
        /// Project name and name of created folder
        project_name: String,

        /// Language. Can be 'c', 'c++', or 'cpp'
        #[arg(long)]
        language: String,

        /// Use a library template instead of the default binary
        #[arg(long, default_value_t = false)]
        lib: bool,

        /// Compiler for selected language (sets C compiler when language is C, C++ when language is C++)
        #[arg(long)]
        compiler: Option<String>,

        /// C++ compiler
        #[arg(long, default_value = "g++")]
        cpp_compiler: String,

        /// C compiler
        #[arg(long, default_value = "gcc")]
        c_compiler: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => app::build(),
        Some(Commands::Run) => {
            let config = io::load_config("Seastar.toml");
            if config.package.is_lib {
                eprintln!("Error: Cannot run a library package");
                exit(1);
            }
            app::run()
        }
        Some(Commands::Clean) => app::clean(),
        Some(Commands::New {
            project_name,
            language,
            lib,
            compiler,
            cpp_compiler,
            c_compiler,
        }) => {
            use std::path::PathBuf;

            let (c_compiler, cpp_compiler) = match language.as_str() {
                "c" => (
                    if &compiler.as_ref().unwrap_or_else(|| &c_compiler) == &c_compiler {
                        cpp_compiler.clone()
                    } else {
                        eprintln!(
                            "Compiler and C compiler don't match. Maybe don't set one of them?"
                        );
                        exit(1);
                    },
                    cpp_compiler.clone(),
                ),
                "cpp" | "c++" => (
                    c_compiler.clone(),
                    if &compiler.as_ref().unwrap_or_else(|| &cpp_compiler) == &cpp_compiler {
                        cpp_compiler.clone()
                    } else {
                        eprintln!(
                            "Compiler and C++ compiler don't match. Maybe don't set one of them?"
                        );
                        exit(1);
                    },
                ),
                _ => (
                    compiler.clone().unwrap_or_else(|| "gcc".to_string()),
                    cpp_compiler.clone(),
                ),
            };

            let copy_to = PathBuf::from(project_name);
            templater::template(
                language,
                *lib,
                &copy_to,
                &c_compiler,
                &cpp_compiler,
                project_name,
            );
        }
        None => {
            println!("Commands: build, run, clean, new");
        }
    }
}
