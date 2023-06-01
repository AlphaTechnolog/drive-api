use std::{path::Path, fs::{self, DirEntry}};

use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleArchive {
    pub kind: &'static str,
    pub filename: String
}

pub struct Archive {
    pub kind: &'static str, // usually DIR or FILE
    pub direntry: DirEntry
}

impl Archive {
    pub fn new(kind: &'static str, direntry: DirEntry) -> Self {
        Self {
            kind,
            direntry
        }
    }

    pub fn to_simple_archive(&self) -> SimpleArchive {
        let filename = self.direntry.file_name();
        let filename_str = filename.to_string_lossy().to_string();
        SimpleArchive {
            kind: self.kind,
            filename: filename_str
        }
    }
}

pub fn list_dir(path: &Path) -> Result<Vec<Archive>, &str> {
    if !path.is_dir() {
        return Err("Given path isn't a folder");
    }

    let mut file_list: Vec<Archive> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    file_list.push(Archive::new("DIR", entry));
                } else {
                    file_list.push(Archive::new("FILE", entry));
                }
            }
        }

        return Ok(file_list);
    }

    Err("Something went wrong for some reason :c")
}