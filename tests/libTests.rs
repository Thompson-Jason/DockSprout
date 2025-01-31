#[cfg(test)]
mod tests {
    use std::process::{Command, ExitStatus, Child, Stdio};
    use std::os::unix::process::ExitStatusExt; // For creating fake exit statuses
    use dock_sprout::{run_docker_compose, run_docker_compose_concurrent};

    #[test]
    fn test_run_docker_compose_mock() {
        let test_files = vec![
            "test1/docker-compose.yml".to_string(),
            "test2/docker-compose.yml".to_string(),
        ];

        let direction_args = vec!["up".to_string(), "-d".to_string()];

        // Mock function to replace the real Docker command
        let mock_runner = |file: &str, _direction_args: &Vec<String>, _verbose: bool| -> std::io::Result<ExitStatus> {
            println!("Mocked execution: docker compose -f {} up -d", file);
            Ok(ExitStatus::from_raw(0))
        };

        run_docker_compose(test_files, direction_args, true, mock_runner);
    }


    #[test]
    fn test_run_docker_compose_concurrent_mock() {
        let test_files = vec![
            "test1/docker-compose.yml".to_string(),
            "test2/docker-compose.yml".to_string(),
        ];

        let direction_args = vec!["up".to_string(), "-d".to_string()];

        // Mock function to replace the real Docker command
        let mock_runner = |file: &str, _direction_args: &Vec<String>, _verbose: bool| -> std::io::Result<Child> {
        println!("Mocked execution: docker compose -f {} up -d", file);
        Command::new("sleep")
            .arg("1") // A short-lived command that immediately exits
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn() // Returns a Child process
        };

        run_docker_compose_concurrent(test_files, direction_args, true, mock_runner);
    }
}

