use crate::project_state::ProjectState;
use colored::Colorize;
use std::path::Path;
pub fn handle_import(project_name: &str, file_name: &str) {
    let project_state = ProjectState::load();
    let default_path = project_state.path();
    let path = Path::new(&default_path).join(file_name);
    if !path.exists() {
        println!("{}", "Oops, I can't seem to find this file".red());
        return;
    }
    if let Some(project_path) = project_state.get_project_path(project_name) {
        let project_path = Path::new(project_path);
        let project_file = project_path.join(file_name);
        if project_file.exists() {
            println!(
                "{} {}",
                "File already exists in project folder:".red(),
                project_name
            );
            return;
        }
        std::fs::copy(&path, &project_file).unwrap();
        println!(
            "{} {}",
            "File imported to project:".green(),
            project_name.bold(),
        );
    } else {
        println!("{}", "Seems like project you entered doesn't exist".red());
    }
}
