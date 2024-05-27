use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
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
    fn get_downloads_path() -> String {
        let home_dir = dirs::home_dir().unwrap();
        format!("{}/Downloads", home_dir.display())
    }
    fn new() -> ProjectState {
        ProjectState {
            default_path: ProjectState::get_downloads_path(),
            projects: HashMap::new(),
        }
    }
    pub fn get_storage_path() -> PathBuf {
        let home_dir = home_dir().expect("Failed to get home directory");
        let mut config_dir = PathBuf::from(&home_dir);
        config_dir.push(".assetm");
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        config_dir.push("projects.json");
        config_dir
    }
    fn save(&self) {
        let state = serde_json::to_string(&self).unwrap();
        let path = ProjectState::get_storage_path();
        std::fs::write(path, state).unwrap();
    }

    pub fn load() -> ProjectState {
        let path_to_save = ProjectState::get_storage_path();
        if let Ok(state) = std::fs::read_to_string(path_to_save) {
            serde_json::from_str(&state).unwrap_or_else(|_| ProjectState::new())
        } else {
            ProjectState::new()
        }
    }
    pub fn projects(&self) -> &HashMap<String, String> {
        &self.projects
    }
    pub fn path(&self) -> &str {
        &self.default_path
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
    pub fn remove_project(&mut self, project_name: &str) -> Result<(), ProjectError> {
        if self.projects.remove(project_name).is_none() {
            return Err(ProjectError::ProjectNotFound);
        }
        self.save();
        Ok(())
    }
    pub fn set_path(&mut self, path: &str) {
        self.default_path = path.to_string();
        self.save();
    }
    pub fn get_project_path(&self, project_name: &str) -> Option<&String> {
        self.projects.get(project_name)
    }
}
