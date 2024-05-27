use crate::project_state;
use colored::Colorize;
pub fn handle_default() {
    let mut project_state = project_state::ProjectState::load();
    let current_dir = std::env::current_dir().unwrap().display().to_string();
    project_state.set_path(&current_dir);
    println!("Default path is set to: {}", current_dir.green());
}
pub fn print_default() {
    let project_state = project_state::ProjectState::load();
    let path = project_state.path();
    println!("Default path is: {}", path.bright_blue().bold());
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_handle_default() {
        let mut project_state = project_state::ProjectState::load();
        let current_dir = std::env::current_dir().unwrap().display().to_string();
        project_state.set_path(&current_dir);
        handle_default();
        let project_state = project_state::ProjectState::load();
        assert_eq!(project_state.path(), current_dir);
    }
}
