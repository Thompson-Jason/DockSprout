use std::path::{Path};
fn is_allowed_root(path: &Path) -> bool {
    // Allow override for testing or admin use
    if std::env::var("SPROUT_ALLOW_ANY_ROOT").map(|v| v == "1" || v.to_lowercase() == "true").unwrap_or(false) {
        return true;
    }
    if let Some(home) = dirs::home_dir() {
        if path.starts_with(&home) {
            return true;
        }
    }
    if let Some(tmp) = std::env::temp_dir().to_str() {
        if path.starts_with(tmp) {
            return true;
        }
    }
    false
}
use ignore::{Walk, WalkBuilder};
use std::env;
use log::{info, error, warn};

fn build_walker(path: &Path) -> Walk {
    let require_git = env::var("SPROUT_REQUIRE_GIT").map(|v| v == "1" || v.to_lowercase() == "true").unwrap_or(false);
    let mut walker = WalkBuilder::new(path);
    walker
        .standard_filters(false)
        // .parents(true)
        .hidden(false)
        .require_git(require_git)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        // Uses .sprout-ignore for custom ignore patterns
        .add_custom_ignore_filename(".sprout-ignore");

    walker.build()
}

pub fn get_compose_filepaths(root: &Path) -> Vec<String> {
    if !is_allowed_root(root) {
        error!("Directory traversal denied: {:?} is not under $HOME or /tmp. Set SPROUT_ALLOW_ANY_ROOT=1 to override.", root);
        return Vec::new();
    }
    let mut paths = Vec::new();
    let walker = build_walker(root);

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if let Some(file_stem) = path.file_stem() {
                    if file_stem == "docker-compose" && (path.extension().map_or(false, |ext| ext == "yaml") || path.extension().map_or(false, |ext| ext == "yml")) {
                        info!("Found compose file: {}", path.display());
                        paths.push(path.to_string_lossy().to_string());
                    }
                }
            },
            Err(e) => {
                warn!("Walker error: {}", e);
            }
        }
    }
    paths
}
