use crate::project_state::ProjectState;
use colored::Colorize;
pub fn handle_remove(project_name: &str) {
    let mut project_service = ProjectState::load();
    match project_service.remove_project(&project_name) {
        Ok(_) => println!("{}", "Project removed successfuly.".green()),
        Err(_) => println!("{}", "Project not found!".red()),
    }
}
