#[cfg(test)]
mod tests {
    use std::process::{Command, ExitStatus, Child, Stdio};
    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt; // For creating fake exit statuses
    #[cfg(windows)]
    use std::os::windows::process::ExitStatusExt;
    use dock_sprout::{run_docker_compose, run_docker_compose_concurrent};

    #[test]
    fn test_run_docker_compose_mock() {
        let test_files = vec![
            "test1/docker-compose.yml".to_string(),
            "test2/docker-compose.yml".to_string(),
        ];

        let direction_args = vec!["up".to_string(), "-d".to_string()];

        // Mock function to replace the real Docker command
        let mock_runner = |file: &str, _direction_args: &[String], _verbose: bool| -> std::io::Result<ExitStatus> {
            println!("Mocked execution: docker compose -f {} up -d", file);
            #[cfg(unix)]
            { Ok(ExitStatus::from_raw(0)) }
            #[cfg(windows)]
            { Ok(ExitStatus::from_raw(0)) }
        };

        run_docker_compose(test_files, &direction_args, true, mock_runner);
    }


    #[test]
    fn test_run_docker_compose_concurrent_mock() {
        let test_files = vec![
            "test1/docker-compose.yml".to_string(),
            "test2/docker-compose.yml".to_string(),
        ];

        let direction_args = vec!["up".to_string(), "-d".to_string()];

        // Mock function to replace the real Docker command
        let mock_runner = |file: &str, _direction_args: &[String], _verbose: bool| -> std::io::Result<Child> {
            println!("Mocked execution: docker compose -f {} up -d", file);
            // Use a cross-platform short-lived command
            #[cfg(unix)]
            {
                Command::new("sleep")
                    .arg("1")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            }
            #[cfg(windows)]
            {
                Command::new("cmd")
                    .arg("/C")
                    .arg("timeout /T 1 > NUL")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            }
        };

        run_docker_compose_concurrent(test_files, &direction_args, true, mock_runner);
    }
}

