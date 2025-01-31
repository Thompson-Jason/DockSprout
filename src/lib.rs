pub mod walker;

pub fn run_docker_compose<F>(files: Vec<String>, direction_args: Vec<String>, mut command_runner: F)
where
    F: FnMut(&str, &Vec<String>) -> std::io::Result<std::process::ExitStatus>,
{
    for file_path in files.iter() {

        command_runner(&file_path, &direction_args).unwrap();
    }
}

