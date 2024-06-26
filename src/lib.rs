use clap::Parser;
use colored::*;
use std::fs;
use std::process::{Command, Stdio};
use std::{
    error::Error,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(version = "0.2.4",author = "NazrinDuck", about, long_about = None)]
pub struct Cli {
    pub files_name: Vec<String>,

    #[arg(short = 'g', long = "debug", action = clap::ArgAction::Count)]
    pub debug: u8,
    #[arg(short = 's', long = "strict")]
    pub is_strict: bool,
    #[arg(short = 'd', long = "detail")]
    pub is_detailed: bool,
    #[arg(short = 'r', long = "run")]
    pub is_run: bool,
    #[arg(short = 'H', long = "history")]
    pub use_history: bool,
}

struct File {
    file_path: PathBuf,
    file_stem: String,
    file_extension: String,
    output_folder: Option<PathBuf>,
}

impl File {
    fn new(file_path: PathBuf, file_stem: String, file_extension: String) -> Self {
        File {
            file_path,
            file_stem,
            file_extension,
            output_folder: None,
        }
    }

    fn set_folder(self: &mut File, folder: PathBuf) {
        self.output_folder = Some(folder);
    }

    fn get_folder(self: &File) -> PathBuf {
        self.output_folder.clone().unwrap()
    }
}

pub fn run(
    file_path: &Path,
    debug: u8,
    is_detailed: bool,
    is_run: bool,
) -> Result<(), Box<dyn Error>> {
    let mut file: File = parse_file(file_path)?;
    let cmd_str: String = analyse_extension(&mut file, debug)?;

    if is_detailed {
        todo!()
        //wait for next version
    } else {
        compile(cmd_str)?;
    }

    if is_run {
        run_file(&file)?;
    }
    Ok(())
}

fn parse_file(file_path: &Path) -> Result<File, Box<dyn Error>> {
    if !file_path.try_exists()? {
        return Err(format!("{} don't exist!", file_path.to_str().unwrap()).into());
    };

    let path: PathBuf = fs::canonicalize(file_path.parent().unwrap())?;

    let file_stem = file_path
        .file_stem()
        .ok_or("No file stem found!")?
        .to_str()
        .unwrap();

    let file_extension = file_path
        .extension()
        .ok_or("No extension found!")?
        .to_str()
        .unwrap();

    Ok(File::new(
        path,
        file_stem.to_string(),
        file_extension.to_string(),
    ))
}

fn analyse_extension(file: &mut File, debug: u8) -> Result<String, Box<dyn Error>> {
    let mut out_path: PathBuf = file.file_path.clone();
    let cmd_str: String;
    let c_flags: &str = match debug {
        0_u8 => "-O3 -Wall -Wextra -lm -masm=intel ",
        1_u8 => "-O1 -g -no-pie -lm -masm=intel ",
        2_u8 => "-O0 -g -no-pie -fno-stack-protector -z execstack -lm -masm=intel ",
        3_u8 => "-O0 -g -no-pie -fno-stack-protector -z execstack -z norelro -lm -masm=intel ",
        _ => return Err("debug flags is limitd to 0~3!".into()),
    };

    /*
    {
        "-O0 -g -masm=intel -no-pie -fno-stack-protector"
    } else {
        "-O3 -Wall -Wextra -lm"
    };
    */

    match file.file_extension.as_str() {
        "c" => {
            out_path.push("c-output");
            if !out_path.try_exists()? {
                fs::create_dir(&out_path)?;
            }

            file.set_folder(out_path);
            cmd_str = String::from(format!(
                "gcc {dir}/{name}.c -o {dir}/c-output/{name}.out {c_flags}",
                dir = file.file_path.display(),
                name = file.file_stem,
            ));
        }
        "cpp" => {
            out_path.push("cpp-output");
            if !out_path.try_exists()? {
                fs::create_dir(&out_path)?;
            }

            file.set_folder(out_path);
            cmd_str = String::from(format!(
                "g++ {dir}/{name}.cpp -o {dir}/cpp-output/{name}.out {c_flags}",
                dir = file.file_path.display(),
                name = file.file_stem,
            ));
        }
        other => {
            return Err(format!(
                ".{} file is not supported yet or it can not be complied",
                other
            )
            .into());
        }
    };
    Ok(cmd_str)
}

fn compile(cmd_str: String) -> Result<(), Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &cmd_str]).output()?
    } else {
        Command::new("sh").arg("-c").arg(&cmd_str).output()?
    };

    let err: String = String::from_utf8(output.stderr)?;
    if output.status.success() {
        println!("quick compiling successfully!");
        if !err.is_empty() {
            println!("{}: {}", "[Warning]".yellow(), err);
        }
        Ok(())
    } else {
        Err(err.into())
    }
}

fn run_file(file: &File) -> Result<(), Box<dyn Error>> {
    println!("running code...");

    let mut path: PathBuf = file.get_folder();
    path.push(format!("{}.out", &file.file_stem));
    let cmd_str = path.to_str().unwrap();

    println!("=====================input=====================");
    println!("(Press Ctrl+D to quit)");

    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let input: String = String::from_utf8(input)?;

    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &cmd_str])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&cmd_str)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?
    };

    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(input.as_bytes()).unwrap();
    });

    println!("=====================output====================");
    let output = child.wait_with_output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    println!("======================End======================");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{compile, run};
    use std::path::Path;

    #[test]
    fn test_run() {
        assert!(run(Path::new("./test1.c"), 1, true, true).is_err());
        assert!(run(Path::new("./test2.cpp"), 1, true, true).is_err());
        assert!(run(Path::new("./test2"), 1, true, true).is_err());
        assert!(run(Path::new("./test2.a"), 1, true, true).is_err());
    }
    #[test]
    #[should_panic]
    fn test_panic() {
        run(Path::new("./.a"), 0, true, true).unwrap();
    }
    #[test]
    fn test_compile_quickly() {
        //compile_quickly(String::new());
    }
    /*
     */
}
