#[cfg(test)]
mod tests {

            #[test]
            fn test_empty_directory() {
                let temp_dir = tempdir().unwrap();
                let found_files = get_compose_filepaths(temp_dir.path());
                assert!(found_files.is_empty(), "No files should be found in an empty directory");
            }

            #[test]
            fn test_symlinked_compose_file() {
                use std::os::unix::fs::symlink;
                let temp_dir = tempdir().unwrap();
                let real_dir = temp_dir.path().join("real");
                let link_dir = temp_dir.path().join("link");
                create_dir_all(&real_dir).unwrap();
                create_dir_all(&link_dir).unwrap();
                let compose_file_path = real_dir.join("docker-compose.yml");
                File::create(&compose_file_path).unwrap();
                let symlink_path = link_dir.join("docker-compose.yml");
                symlink(&compose_file_path, &symlink_path).unwrap();
                let found_files = get_compose_filepaths(temp_dir.path());
                assert!(found_files.iter().any(|f| f.ends_with("docker-compose.yml")), "Should find symlinked compose file");
            }


            #[test]
            fn test_sprout_ignore_complex_patterns() {
                let temp_dir = tempdir().unwrap();
                let docker_dir1 = temp_dir.path().join("project1");
                let docker_dir2 = temp_dir.path().join("project2");
                let docker_dir3 = temp_dir.path().join("project3");
                create_dir_all(&docker_dir1).unwrap();
                create_dir_all(&docker_dir2).unwrap();
                create_dir_all(&docker_dir3).unwrap();
                let compose_file_path1 = docker_dir1.join("docker-compose.yml");
                let compose_file_path2 = docker_dir2.join("docker-compose.yml");
                let compose_file_path3 = docker_dir3.join("docker-compose.yml");
                File::create(&compose_file_path1).unwrap();
                File::create(&compose_file_path2).unwrap();
                File::create(&compose_file_path3).unwrap();
                let ignore_file_path = temp_dir.path().join(".sprout-ignore");
                // Ignore project2 and any directory starting with 'project3'
                let _ = File::create(&ignore_file_path).unwrap().write_all(b"project2/*\nproject3*");
                let found_files = get_compose_filepaths(temp_dir.path());
                assert!(found_files.contains(&compose_file_path1.to_string_lossy().to_string()), "project1 should not be ignored");
                assert!(!found_files.contains(&compose_file_path2.to_string_lossy().to_string()), "project2 should be ignored");
                assert!(!found_files.contains(&compose_file_path3.to_string_lossy().to_string()), "project3 should be ignored by pattern");
            }
        use std::os::unix::fs::PermissionsExt;

        #[test]
        fn test_permission_error() {
            let temp_dir = tempdir().unwrap();
            let restricted_dir = temp_dir.path().join("restricted");
            create_dir_all(&restricted_dir).unwrap();
            let compose_file_path = restricted_dir.join("docker-compose.yml");
            File::create(&compose_file_path).unwrap();
            // Remove read permissions from the directory
            let mut perms = std::fs::metadata(&restricted_dir).unwrap().permissions();
            perms.set_mode(0o000);
            std::fs::set_permissions(&restricted_dir, perms.clone()).unwrap();
            // Should not panic, should not find the file
            let found_files = get_compose_filepaths(temp_dir.path());
            assert!(!found_files.contains(&compose_file_path.to_string_lossy().to_string()), "Should not find file in unreadable directory");
            // Restore permissions for cleanup
            perms.set_mode(0o700);
            std::fs::set_permissions(&restricted_dir, perms).unwrap();
        }

        #[test]
        fn test_deeply_nested_directories() {
            let temp_dir = tempdir().unwrap();
            let mut deep_dir = temp_dir.path().to_path_buf();
            for i in 0..10 {
                deep_dir = deep_dir.join(format!("level{}", i));
                create_dir_all(&deep_dir).unwrap();
            }
            let compose_file_path = deep_dir.join("docker-compose.yml");
            File::create(&compose_file_path).unwrap();
            let found_files = get_compose_filepaths(temp_dir.path());
            assert!(found_files.contains(&compose_file_path.to_string_lossy().to_string()), "Should find file in deeply nested directory");
        }

        #[test]
        fn test_invalid_compose_file_name() {
            let temp_dir = tempdir().unwrap();
            let docker_dir = temp_dir.path().join("project1");
            create_dir_all(&docker_dir).unwrap();
            let invalid_file_path = docker_dir.join("docker-compose.txt");
            File::create(&invalid_file_path).unwrap();
            let found_files = get_compose_filepaths(temp_dir.path());
            assert!(!found_files.contains(&invalid_file_path.to_string_lossy().to_string()), "Should not detect non-yml compose file");
        }
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

