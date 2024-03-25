use clap::Parser;
use std::fs;
use std::process::Command;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(version = "0.1.0",author = "NazrinDuck", about, long_about = None)]
pub struct Cli {
    pub files_name: Vec<String>,

    #[arg(short = 'q', long = "quick")]
    pub is_quick: bool,
}

pub fn run(file_path: &Path, is_quick: bool) -> Result<(), Box<dyn Error>> {
    let cmd_str = parse_file(file_path)?;

    println!("{}", cmd_str);

    if is_quick {
        compile_quickly(cmd_str)?
    } else {
    }
    Ok(())
}

fn parse_file(file_path: &Path) -> Result<String, Box<dyn Error>> {
    if !file_path.try_exists()? {
        return Err(format!("{} don't exist!", file_path.to_str().unwrap()).into());
    };

    let path: PathBuf = fs::canonicalize(file_path.parent().unwrap())?;
    let mut out_path: PathBuf = path.clone();
    let cmd_str: String;

    let file_stem = file_path
        .file_stem()
        .expect("[Error]: No file stem found!")
        .to_str()
        .unwrap();

    match file_path
        .extension()
        .expect("[Error]: No extension found!")
        .to_str()
    {
        Some("c") => {
            out_path.push("c-output");
            if !out_path.try_exists()? {
                fs::create_dir(out_path)?;
            }

            cmd_str = String::from(format!(
                "gcc -O3 -Wall -Wextra {dir}/{name}.c -o {dir}/c-output/{name}.out",
                dir = path.display(),
                name = file_stem,
            ));
        }
        Some("cpp") => {
            out_path.push("cpp-output");
            if !out_path.try_exists()? {
                fs::create_dir(out_path)?;
            }

            cmd_str = String::from(format!(
                "g++ -O3 -Wall -Wextra {dir}/{name}.c -o {dir}/cpp-output/{name}.out",
                dir = path.display(),
                name = file_stem,
            ));
        }
        Some(other) => {
            return Err(format!(
                ".{} file is not supported yet or it can not be complied",
                other
            )
            .into());
        }
        None => {
            return Err("Extension analysing error!".into());
        }
    };
    Ok(cmd_str)
}

fn compile_quickly(cmd_str: String) -> Result<(), Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &cmd_str]).output()?
    } else {
        Command::new("sh").arg("-c").arg(&cmd_str).output()?
    };

    let err: String = String::from_utf8(output.stderr)?;
    if output.status.success() {
        println!("quick compiling successfully!");
        if !err.is_empty() {
            println!("[Waring]: {}", err);
        }
        Ok(())
    } else {
        Err(err.into())
    }
}

fn run_file() {}

#[cfg(test)]
mod tests {
    use crate::{compile_quickly, run};
    use std::path::Path;

    #[test]
    fn test_run() {
        assert!(run(Path::new("./test1.c"), true).is_err());
        assert!(run(Path::new("./test2.cpp"), true).is_err());
        assert!(run(Path::new("./test2"), true).is_err());
        assert!(run(Path::new("./test2.a"), true).is_err());
    }
    #[test]
    #[should_panic]
    fn test_panic() {
        run(Path::new("./.a"), true).unwrap();
    }
    #[test]
    fn test_compile_quickly() {
        //compile_quickly(String::new());
    }
}
