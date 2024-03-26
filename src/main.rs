use clap::Parser;
use colored::*;
use std::path::Path;
use std::process;

use rmake::Cli;

fn main() {
    let cli: Cli = Cli::parse();

    for file_name in cli.files_name {
        let file_path: &Path = Path::new(&file_name);

        rmake::run(file_path, cli.is_detailed, cli.is_run).unwrap_or_else(|err| {
            eprintln!("{}: {}", "[Error]".red(), err);
            process::exit(1);
        });
    }
}
