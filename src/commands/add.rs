use colored::Colorize;

use crate::project_state::ProjectState;
pub fn handle_add(project_name: &str) {
    let project_name = project_name.to_string();
    let current_dir = std::env::current_dir().unwrap().display().to_string();
    let mut project_service = ProjectState::load();
    match project_service.add_project(&project_name, &current_dir) {
        Ok(_) => println!("{} {}", "Project added:".green(), project_name),
        Err(_) => println!("{} {}", "Project already exists:".red(), project_name),
    }
}
