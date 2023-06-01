use rocket::State;
use crate::path::pth_by_dir;
use crate::reader::{SimpleArchive, list_dir};
use crate::error::SimpleError;
use crate::config::Config;

pub struct Fs<'a> {
    pub dirname: String,
    pub config: &'a State<Config>
}

impl Fs<'_> {
    pub fn list(&self) -> Result<Vec<SimpleArchive>, SimpleError> {
        let path = pth_by_dir(&self.dirname, &self.config);
        match list_dir(path.as_path()) {
            Ok(files_list) => {
                let mut result: Vec<SimpleArchive> = Vec::new();
                for archive in files_list {
                    result.push(archive.to_simple_archive());
                }

                Ok(result)
            },
            Err(err) => {
                Err(SimpleError::new(err.to_string()))
            }
        }
    }
}
