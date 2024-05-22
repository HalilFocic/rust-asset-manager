use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum ProjectError {
    ProjectNotFound,
    ProjectAlreadyExists,
}
#[derive(Serialize, Deserialize)]
pub struct ProjectState {
    default_path: String,
    projects: HashMap<String, String>,
}

impl ProjectState {
    fn new() -> ProjectState {
        ProjectState {
            default_path: String::from("~/Downloads"),
            projects: HashMap::new(),
        }
    }
    fn save(&self) {
        let state = serde_json::to_string(&self).unwrap();
        std::fs::write("projects.json", state).unwrap();
    }

    pub fn load() -> ProjectState {
        if let Ok(state) = std::fs::read_to_string("projects.json") {
            serde_json::from_str(&state).unwrap_or_else(|_| ProjectState::new())
        } else {
            ProjectState::new()
        }
    }
    pub fn add_project(&mut self, project_name: &str, path: &str) -> Result<(), ProjectError> {
        let project_name = project_name.to_string();
        let path = path.to_string();

        if self.projects.contains_key(&project_name) {
            return Err(ProjectError::ProjectAlreadyExists);
        }
        self.projects.insert(project_name, path);
        self.save();
        Ok(())
    }
    pub fn projects(&self) -> &HashMap<String, String> {
        &self.projects
    }
}
