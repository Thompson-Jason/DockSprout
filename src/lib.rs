pub mod walker;

pub fn run_docker_compose<F>(files: Vec<String>, direction_args: Vec<String>, mut command_runner: F)
where
    F: FnMut(&str, &Vec<String>) -> std::io::Result<std::process::Output>,
{
    for file_path in files.iter() {

        let output = command_runner(&file_path, &direction_args).unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("stdout {}", stdout);

        let stderr = String::from_utf8(output.stderr).unwrap();
        println!("stderr {}", stderr);

        println!("status {}", output.status);
    }
}

