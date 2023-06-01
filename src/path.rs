use std::path::{PathBuf, Path};
use rocket::State;
use crate::Config;

pub fn pth_by_dir(dirname: &str, config: &State<Config>) -> PathBuf {
    let dirbinding = dirname.replace("~", "/");
    let dirname = dirbinding.trim_start_matches("/");

    let store_path_filename = config.store_folder.clone();
    let mut path = PathBuf::new();

    path.push(store_path_filename.as_str());
    path.push(Path::new(&dirname));

    path
}