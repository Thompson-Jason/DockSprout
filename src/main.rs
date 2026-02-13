mod walker;
mod signal;

use gumdrop::Options;
use indicatif::{ProgressBar, ProgressStyle};
use std::{env};
use std::time::Duration;
use std::path::PathBuf;
use std::process::{Command, Stdio, ExitStatus, Child};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use log::error;
// ...existing code...



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

fn real_docker_runner(file_path: &str, direction_args: &[String], verbose: bool) -> std::io::Result<ExitStatus> {
    
    if file_path.contains(';') || file_path.contains('&') || file_path.contains('|') || file_path.contains('`') || file_path.contains('$') {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid characters in file path"));
    }
    if verbose {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(file_path)
            .args(direction_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    } else {
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

fn real_docker_runner_concurrent(file_path: &str, direction_args: &[String], verbose: bool) -> std::io::Result<Child> {
    // Security: Validate file_path to prevent command injection
    if file_path.contains(';') || file_path.contains('&') || file_path.contains('|') || file_path.contains('`') || file_path.contains('$') {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid characters in file path"));
    }
    if verbose {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(file_path)
            .args(direction_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
    } else {
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
    env_logger::init();
    if env::args().any(|arg| arg == "-v" || arg == "--version") {
        println!("DockSprout {VERSION}");
        std::process::exit(0);
    }

    let args = Opts::parse_args_default_or_exit();
    let root = match args.source {
        Some(r) => r,
        None => {
            error!("No source directory provided.");
            std::process::exit(1);
        }
    };
    let option = args.option.to_lowercase();
    let direction_args = if option != "up" && option != "down" && option != "pull" {
        error!("Docker Compose direction has to be one of the following (up|down|pull). Argument given = {}", option);
        std::process::exit(1);
    } else if option == "up" {
        vec![option, "-d".to_string()]
    } else {
        vec![option]
    };

    let files = walker::get_compose_filepaths(&root);

    if files.is_empty() {
        error!("No docker-compose.yml files found in {:?}", root);
        std::process::exit(1);
    }

    // Spinner is used for visual feedback, but does not track per-file progress
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["\u{280b}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283c}", "\u{2834}", "\u{2826}", "\u{2827}", "\u{2807}", "\u{280f}"])
            .template("{spinner:.blue} Running {msg}...")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    if args.concurrent {
        // ...existing code...
        let children = Arc::new(Mutex::new(Vec::new()));
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        signal::setup_signal_handler(Arc::clone(&children), Arc::clone(&shutdown_flag));
        let errors = dock_sprout::run_docker_compose_concurrent_collect(files, &direction_args, args.verbose, real_docker_runner_concurrent, Arc::clone(&children), Arc::clone(&shutdown_flag));
        spinner.finish_and_clear();
        if !errors.is_empty() {
            error!("Some docker compose commands failed:");
            for (file, err) in errors {
                error!("  {}: {}", file, err);
            }
            std::process::exit(2);
        }
    } else {
        let errors = dock_sprout::run_docker_compose_collect(files, &direction_args, args.verbose, real_docker_runner);
        spinner.finish_and_clear();
        if !errors.is_empty() {
            error!("Some docker compose commands failed:");
            for (file, err) in errors {
                error!("  {}: {}", file, err);
            }
            std::process::exit(2);
        }
    }
}

