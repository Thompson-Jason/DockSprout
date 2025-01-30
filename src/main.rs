mod walker;

use gumdrop::Options;
use std::{env};
use std::path::PathBuf;
use std::process::{Command, Stdio, Output};



const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Options)]
struct Opts {
    #[options(help = "Display program help")]
    help: bool,

    #[options(help = "Display version information")]
    version: bool,

    #[options(help = "Get docker-compose files from this source recursivly", free, required)]
    source: Option<PathBuf>,

    #[options(help = "docker-compose option (one of: up|down)", free, required)]
    direction: String,

}

fn run_docker_compose<F>(files: Vec<String>, direction_args: Vec<String>, mut command_runner: F)
where
    F: FnMut(&str, &Vec<String>) -> std::io::Result<std::process::Output>,
{
    for file_path in files.iter() {
        
        let output = command_runner(&file_path, &direction_args).unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("stdout {}", stdout);

        let stderr = String::from_utf8(output.stderr).unwrap();
        println!("stderr {}", stderr);

        println!("status {}", output.status);
    }
}

fn real_docker_runner(file_path: &str, direction_args: &Vec<String>) -> std::io::Result<Output> {
    return Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg(file_path)
        .args(direction_args)
        .stdout(Stdio::piped())
        .output();
        //.unwrap();
}

fn main() {
    if env::args().any(|arg| arg == "-v" || arg == "--version") {
        println!("DockerSprout {VERSION}");
        std::process::exit(0);
    }


    let args = Opts::parse_args_default_or_exit();
    let root = args.source.unwrap();
    let direction = args.direction.to_lowercase();
    let mut direction_args = vec![];


    if direction != "up" && direction != "down" {
        eprintln!("Docker Compose direction has to be one of the following (up|down). Argument given = {}", direction);
        std::process::exit(1);
    }else if direction == "up" {
        direction_args = vec![direction, "-d".to_string()];
    }else if direction == "down" {
        direction_args = vec![direction];
    }

    let files = walker::get_compose_filepaths(&root);

    run_docker_compose(files, direction_args, real_docker_runner);
}

