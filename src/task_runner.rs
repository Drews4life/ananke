use std::path::PathBuf;
// TODO: Use tokio alternative `tokio::process:Command;`
use std::process::Command;
use std::fs::canonicalize;
use ananke::{get_child_path_from_string, create_branch_name};

pub struct TaskRunner {}

impl TaskRunner {
    pub fn git_clone(repo_url: &str) {
        Self::spawn(
            Command::new("git")
                .arg("clone")
                .arg(repo_url),
            Option::None
        );
    }

    pub fn git_fetch(project_name: &str) {
        let path = get_child_path_from_string(project_name);

        Self::spawn(
            Command::new("git").arg("fetch").arg("--all"),
            Some(&path)
        );
    }

    pub fn git_pull(project_name: &str) {
        let path = get_child_path_from_string(project_name);

        Self::spawn(
            Command::new("git").arg("pull"),
            Some(&path)
        );
    }

    pub fn git_checkout(project_name: &str, requested_branch: &str, new: bool) {
        let path = get_child_path_from_string(project_name);
        let current_branch = &Self::spawn_with_output(
            Command::new("git")
                .arg("rev-parse")
                .arg("--abbrev-ref")
                .arg("HEAD"),
            Some(&path)
        );

        if !Self::does_local_ver_match(requested_branch, current_branch) {
            let mut args = vec!["checkout".to_owned()];

            if new {
                args.append(&mut vec!["-b".to_owned(), create_branch_name()]);
            }

            Self::spawn(
                Command::new("git").args(args),
                Some(&path)
            );
        }
    }

    pub fn npm_install(project_name: &str) {
        let path = get_child_path_from_string(project_name);

        Self::spawn(
            Command::new("npm").arg("i"),
            Some(&path),
        );
    }

    pub fn npm_run(project_name: &str) {
        let path = get_child_path_from_string(project_name);

        Self::spawn(
            Command::new("npm").arg("start"),
            Some(&path),
        );
    }

    fn does_local_ver_match(requested_branch: &str, current_branch: &str) -> bool {
        requested_branch.eq(current_branch)
    }

    fn spawn(command: &mut Command, at: Option<&PathBuf>) {
        let command = Self::get_command_with_path(command, at);

        command
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn spawn_with_output(command: &mut Command, at: Option<&PathBuf>) -> String {
        let command = Self::get_command_with_path(command, at);

        String::from_utf8(
            command
                .output()
                .unwrap()
                .stdout
        ).unwrap()
    }

    fn get_command_with_path<'a>(command: &'a mut Command, at: Option<&PathBuf>) -> &'a mut Command {
        match at {
            Some(path) => command.current_dir(canonicalize(path).unwrap()),
            None => command,
        }
    }
}