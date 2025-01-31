#[cfg(test)]
mod tests {
    use std::fs::{create_dir_all, File};
    use tempfile::tempdir;
    use dock_sprout::walker::get_compose_filepaths;
    use std::io::Write;

    #[test]
    fn test_find_docker_compose_files() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let docker_dir = temp_dir.path().join("project1");
        create_dir_all(&docker_dir).unwrap();

        // Create a dummy docker-compose.yml file
        let compose_file_path = docker_dir.join("docker-compose.yml");
        File::create(&compose_file_path).unwrap();

        // Call function and check results
        let found_files = get_compose_filepaths(temp_dir.path());

        assert!(found_files.contains(&compose_file_path.to_string_lossy().to_string()), 
            "docker-compose.yml should be detected");
    }

    #[test]
    fn test_compose_ignore(){

        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let docker_dir = temp_dir.path().join("project1");
        let ignore_dir = temp_dir.path().join("project2");
        create_dir_all(&docker_dir).unwrap();
        create_dir_all(&ignore_dir).unwrap();

        // Create a dummy docker-compose.yml file
        let compose_file_path1 = docker_dir.join("docker-compose.yml");
        let compose_file_path2 = ignore_dir.join("docker-compose.yml");

        // Create a dummy .compose-ignore file
        let ignore_file_path = temp_dir.path().join(".sprout-ignore");

        File::create(&compose_file_path1).unwrap();
        File::create(&compose_file_path2).unwrap();
        let _ = File::create(&ignore_file_path).unwrap().write_all(b"project2/*");

        // Call function and check results
        let found_files = get_compose_filepaths(temp_dir.path());

        assert!(!found_files.contains(&compose_file_path2.to_string_lossy().to_string()));
    }
}

