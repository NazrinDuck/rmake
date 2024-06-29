// Copyright (c) 2024 NazrinDuck. All Rights Reserved.
use clap::Parser;
use colored::*;
use std::path::Path;
use std::process;

use rmake::Cli;

fn main() {
    let cli: Cli = Cli::parse();

    if cli.files_name.is_empty() {
        eprintln!("{}: {}", "[Error]".red(), "Please input files to compile");
        eprintln!();
        eprintln!("For more useful information, please use --help or -h");
    }

    for file_name in cli.files_name {
        let file_path: &Path = Path::new(&file_name);

        rmake::run(file_path, cli.debug, cli.is_detailed, cli.is_run).unwrap_or_else(|err| {
            eprintln!("{}: {}", "[Error]".red(), err);
            process::exit(1);
        });
    }
}
