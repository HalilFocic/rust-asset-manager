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

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_handle_add() {
        let project_name = "test_project";
        handle_add(&project_name);
        std::thread::sleep(std::time::Duration::from_millis(50));
        let project_state = ProjectState::load();
        println!("{:?}", project_state.projects());
        assert_eq!(project_state.projects().contains_key(project_name), true);
    }
}
