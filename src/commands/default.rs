use crate::project_state;
use colored::Colorize;
pub fn handle_default(default_path: &str) {
    let mut project_state = project_state::ProjectState::load();
    project_state.set_path(default_path);
    println!("Default path is set to: {}", default_path.green());
}
pub fn print_default() {
    let project_state = project_state::ProjectState::load();
    let path = project_state.path();
    println!("Default path is: {}", path.bright_blue().bold());
}
