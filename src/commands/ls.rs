use crate::project_state::ProjectState;
use colored::Colorize;
pub fn handle_ls() {
    let state = ProjectState::load();
    let projects = state.projects();
    if projects.is_empty() {
        println!("No projects found");
        return;
    }
    for (project_name, _) in projects {
        println!(" {} ", project_name.bright_blue().bold());
    }
}
