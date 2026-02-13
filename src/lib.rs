#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;
pub mod walker;

use log::error;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

pub fn run_docker_compose<F>(files: Vec<String>, direction_args: &[String], verbose: bool, mut command_runner: F)
where
    F: FnMut(&str, &[String], bool) -> std::io::Result<std::process::ExitStatus>,
{
    for file_path in files.iter() {
        match command_runner(file_path, direction_args, verbose) {
            Ok(_status) => {},
            Err(e) => error!("Failed to run docker compose for {}: {}", file_path, e),
        }
    }
}

pub fn run_docker_compose_collect<F>(files: Vec<String>, direction_args: &[String], verbose: bool, mut command_runner: F) -> Vec<(String, String)>
where
    F: FnMut(&str, &[String], bool) -> std::io::Result<std::process::ExitStatus>,
{
    let mut errors = Vec::new();
    for file_path in files.iter() {
        match command_runner(file_path, direction_args, verbose) {
            Ok(status) => {
                if !status.success() {
                    errors.push((file_path.clone(), format!("Exited with status: {}", status)));
                }
            },
            Err(e) => errors.push((file_path.clone(), e.to_string())),
        }
    }
    errors
}

pub fn run_docker_compose_concurrent<F>(files: Vec<String>, direction_args: &[String], verbose: bool, mut command_runner: F)
where
    F: FnMut(&str, &[String], bool) -> std::io::Result<std::process::Child>,
{
    for file_path in files.iter() {
        match command_runner(file_path, direction_args, verbose) {
            Ok(_child) => {},
            Err(e) => error!("Failed to run docker compose for {}: {}", file_path, e),
        }
    }
}

pub fn run_docker_compose_concurrent_collect<F>(
    files: Vec<String>,
    direction_args: &[String],
    verbose: bool,
    command_runner: F,
    children: Arc<Mutex<Vec<std::process::Child>>>,
    shutdown_flag: Arc<AtomicBool>,
) -> Vec<(String, String)>
where
    F: FnMut(&str, &[String], bool) -> std::io::Result<std::process::Child> + Send + Clone + 'static,
{
    use std::thread;
    let mut handles = Vec::new();
    let errors = Arc::new(Mutex::new(Vec::new()));
    for file_path in files.iter() {
        let file = file_path.clone();
        let args = direction_args.to_vec();
        let mut runner = command_runner.clone();
        let errors = Arc::clone(&errors);
        let children = Arc::clone(&children);
        let shutdown_flag = Arc::clone(&shutdown_flag);
        let handle = std::thread::spawn(move || {
            if shutdown_flag.load(std::sync::atomic::Ordering::SeqCst) {
                return;
            }
            match runner(&file, &args, verbose) {
                Ok(child) => {
                    let child_id = child.id();
                    {
                        let mut ch = children.lock().unwrap();
                        ch.push(child);
                    }
                    let status = {
                        let mut ch = children.lock().unwrap();
                        let idx = ch.iter().position(|c| c.id() == child_id);
                        // Remove the child from the list before waiting
                        let child_opt = idx.map(|i| ch.remove(i));
                        let status = if let Some(mut child) = child_opt {
                            child.wait()
                        } else {
                            // Should not happen, but fallback
                            #[cfg(unix)]
                            { Ok(std::process::ExitStatus::from_raw(1)) }
                            #[cfg(windows)]
                            { Ok(std::process::ExitStatus::from_raw(1)) }
                        };
                        status
                    };
                    match status {
                        Ok(status) => {
                            if !status.success() {
                                let mut errs = errors.lock().unwrap();
                                errs.push((file.clone(), format!("Exited with status: {}", status)));
                            }
                        },
                        Err(e) => {
                            let mut errs = errors.lock().unwrap();
                            errs.push((file.clone(), e.to_string()));
                        }
                    }
                },
                Err(e) => {
                    let mut errs = errors.lock().unwrap();
                    errs.push((file.clone(), e.to_string()));
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    let errors = Arc::try_unwrap(errors).map(|m| m.into_inner().unwrap()).unwrap_or_else(|arc| (*arc.lock().unwrap()).clone());
    errors
}
