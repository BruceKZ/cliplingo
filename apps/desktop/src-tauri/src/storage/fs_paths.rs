use std::path::PathBuf;

use directories::ProjectDirs;

use crate::services::config::ConfigServiceError;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "brucekz";
const APPLICATION: &str = "cliplingo";

pub trait ConfigPathProvider {
    fn config_path(&self) -> Result<PathBuf, ConfigServiceError>;
    fn history_path(&self) -> Result<PathBuf, ConfigServiceError>;
    fn log_path(&self) -> Result<PathBuf, ConfigServiceError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectPathProvider;

impl ConfigPathProvider for ProjectPathProvider {
    fn config_path(&self) -> Result<PathBuf, ConfigServiceError> {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or(ConfigServiceError::ConfigPathUnavailable)?;

        Ok(project_dirs.config_dir().join("config.toml"))
    }

    fn history_path(&self) -> Result<PathBuf, ConfigServiceError> {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or(ConfigServiceError::ConfigPathUnavailable)?;

        Ok(project_dirs.data_dir().join("history.json"))
    }

    fn log_path(&self) -> Result<PathBuf, ConfigServiceError> {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or(ConfigServiceError::ConfigPathUnavailable)?;

        Ok(project_dirs
            .state_dir()
            .unwrap_or_else(|| project_dirs.data_local_dir())
            .join("app.log"))
    }
}
