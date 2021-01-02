
use {
    anyhow::*,
    directories_next::ProjectDirs,
};

/// return the instance of ProjectDirs holding the app specific paths
pub fn app_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("org", "dystroy", "csv2svg")
        .context("Unable to find app directories")
}
