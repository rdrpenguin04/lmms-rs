use crate::data::Project;
use crate::util::ReadSeek;
use std::io;

#[derive(Debug, Default)]
pub struct Engine {
    pub project: Option<Project>, // TODO: Replace with loading default project
}

impl Engine {
    /// # Errors
    /// Returns Err if there is an error reading the file
    pub fn load_project_file<F: ReadSeek>(&mut self, f: &mut F) -> io::Result<()> {
        self.project = Some(Project::from_file(f)?);
        Ok(())
    }
}
