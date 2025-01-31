use std::path::{Path};
use ignore::{Walk, WalkBuilder};

    fn build_walker(path: &Path) -> Walk {
        let mut walker = WalkBuilder::new(path);
        walker
            .standard_filters(false)
           // .parents(true)
            .hidden(false)
            .require_git(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .add_custom_ignore_filename(".sprout-ignore");
    
        return walker.build();
    }

pub fn get_compose_filepaths(root: &Path) -> Vec<String> {
    let mut paths = Vec::new();
    let walker = build_walker(root);

    for entry in walker {
        if let Err(_e) = entry {
            continue;
        }
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(file_stem) = path.file_stem() {
            if file_stem == "docker-compose" && (path.extension().map_or(false, |ext| ext == "yaml") || path.extension().map_or(false, |ext| ext == "yml")) {
                println!("Path: {}", path.display());
                paths.push(path.to_string_lossy().to_string());
            }
        }
    }
    return paths;
}
