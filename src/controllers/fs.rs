use rocket::State;
use std::path::Path;
use crate::path::pth_by_dir;
use crate::reader::{SimpleArchive, list_dir};
use crate::error::SimpleError;
use crate::log::SimpleMessage;
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

    pub fn read_file(&self) -> Result<String, String> {
        let path = pth_by_dir(&self.dirname, &self.config);
        if path.is_dir() {
            return Err("The given path isn't a file!".to_string());
        }

        let result = std::fs::read(path.as_path());
        if let Err(err) = result {
            return Err(err.to_string());
        }

        if let Ok(bytes) = result {
            if let Ok(v) = std::str::from_utf8(&bytes) {
                return Ok(String::from(v));
            } else {
                return Err("Cannot decode file bytes".to_string());
            }
        }

        Err("Something went wrong unexpectly :c".to_string())
    }

    fn remove_directory(&self, path: &Path) -> Result<SimpleMessage, SimpleError> {
        let filename_str = path.to_string_lossy().to_string();
        match std::fs::remove_dir_all(path) {
            Ok(_) => {
                Ok(SimpleMessage::new(format!("Directory {} were removed successfully!", filename_str)))
            },
            Err(err) => {
                Err(SimpleError::new(format!("Cannot remove directory {}: {}", filename_str, err.to_string())))
            }
        }
    }

    fn remove_file(&self, path: &Path) -> Result<SimpleMessage, SimpleError> {
        let filename_str = path.to_string_lossy().to_string();
        match std::fs::remove_file(path) {
            Ok(_) => {
                Ok(SimpleMessage::new(format!("File {} was successfully removed from the file system", filename_str)))
            },
            Err(err) => {
                Err(SimpleError::new(format!("Cannot remove file {}: {}", filename_str, err.to_string())))
            }
        }
    }

    pub fn remove(&self) -> Result<SimpleMessage, SimpleError> {
        let path = pth_by_dir(&self.dirname, &self.config);
        if path.is_dir() {
            self.remove_directory(path.as_path())
        } else {
            self.remove_file(path.as_path())
        }
    }
}
