use colored::Colorize;

use crate::project_state::ProjectState;
pub fn handle_add(project_name: &str, path_to_project: &str) {
    let project_name = project_name.to_string();
    let project_path = path_to_project.to_string();
    let mut project_service = ProjectState::load();
    match project_service.add_project(&project_name, &project_path) {
        Ok(_) => println!("{} {}", "Project added:".green(), project_name),
        Err(_) => println!("{} {}", "Project already exists:".red(), project_name),
    }
}
