use clap::{Parser, Subcommand};

mod io;
mod builder;
mod language;
mod depman;
mod app;

#[derive(Parser)]
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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => app::build(),
        Some(Commands::Run) => app::run(),
        Some(Commands::Clean) => app::clean(),
        None => println!("Commands: build, run"),
    }
}
