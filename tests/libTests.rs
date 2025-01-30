#[cfg(test)]
mod tests {
    use std::process::{Output, ExitStatus};
    use std::os::unix::process::ExitStatusExt; // For creating fake exit statuses
    use dock_sprout::run_docker_compose;

    #[test]
    fn test_run_docker_compose_mock() {
        let test_files = vec![
            "test1/docker-compose.yml".to_string(),
            "test2/docker-compose.yml".to_string(),
        ];

        let direction_args = vec!["up".to_string(), "-d".to_string()];

        // Mock function to replace the real Docker command
        let mock_runner = |file: &str, _direction_args: &Vec<String>| -> std::io::Result<Output> {
            println!("Mocked execution: docker compose -f {} up -d", file);
            Ok(Output {
                status: ExitStatus::from_raw(0), // Pretend it succeeds
                stdout: b"Mocked Docker output\n".to_vec(),
                stderr: vec![],
            })
        };

        run_docker_compose(test_files, direction_args, mock_runner);
    }
}

