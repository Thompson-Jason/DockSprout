mod walker;

use gumdrop::Options;
use indicatif::{ProgressBar, ProgressStyle};
use std::{env};
use std::time::Duration;
use std::path::PathBuf;
use std::process::{Command, Stdio, ExitStatus, Child};
use dock_sprout::{run_docker_compose, run_docker_compose_concurrent};



const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Options)]
struct Opts {
    #[options(help = "Display program help")]
    help: bool,

    #[options(help = "Display version information")]
    version: bool,

    #[options(help = "Get docker-compose files from this source recursively", free, required)]
    source: Option<PathBuf>,

    #[options(help = "docker-compose option (one of: up|down|pull)", free, required)]
    option: String,

    #[options(help = "Runs the docker compose commands concurrently", default = "false")]
    concurrent: bool,

    #[options(help = "Output docker compose output to stdout", no_short, default = "false")]
    verbose: bool,

}

fn real_docker_runner(file_path: &str, direction_args: &Vec<String>, verbose: bool) -> std::io::Result<ExitStatus> {
    
    if verbose {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(file_path)
            .args(direction_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    }else{
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(file_path)
            .args(direction_args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    }
}

fn real_docker_runner_concurrent(file_path: &str, direction_args: &Vec<String>, verbose: bool) -> std::io::Result<Child> {
    if verbose {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(file_path)
            .args(direction_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
    }else{
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(file_path)
            .args(direction_args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
    }
}

fn main() {
    if env::args().any(|arg| arg == "-v" || arg == "--version") {
        println!("DockSprout {VERSION}");
        std::process::exit(0);
    }


    let args = Opts::parse_args_default_or_exit();
    let root = args.source.unwrap();
    let option = args.option.to_lowercase();
    let mut direction_args = vec![];


    if option != "up" && option != "down" && option != "pull" {
        eprintln!("Docker Compose direction has to be one of the following (up|down|pull). Argument given = {}", option);
        std::process::exit(1);
    }else if option == "up" {
        direction_args = vec![option, "-d".to_string()];
    }else if option == "down" || option == "pull"{
        direction_args = vec![option];
    }

    let files = walker::get_compose_filepaths(&root);

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
    ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.blue} Running {msg}...") // Custom styling
        .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    if args.concurrent {
        run_docker_compose_concurrent(files, direction_args, args.verbose, real_docker_runner_concurrent);
    }else{
        run_docker_compose(files, direction_args, args.verbose, real_docker_runner);
    }
}

