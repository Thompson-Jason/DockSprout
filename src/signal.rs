// Logging and signal handling utilities for DockSprout
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process::Child;
use std::sync::Mutex;
use log::warn;

/// Sets up Ctrl+C handler to terminate all running children gracefully.
pub fn setup_signal_handler(children: Arc<Mutex<Vec<Child>>>, shutdown_flag: Arc<AtomicBool>) {
    let children = Arc::clone(&children);
    let shutdown_flag = Arc::clone(&shutdown_flag);
    ctrlc::set_handler(move || {
        warn!("Received Ctrl+C! Attempting to terminate all child processes...");
        shutdown_flag.store(true, Ordering::SeqCst);
        let mut children = children.lock().unwrap();
        for child in children.iter_mut() {
            let _ = child.kill();
        }
        std::process::exit(130); // 130 = 128 + SIGINT
    }).expect("Error setting Ctrl+C handler");
}
