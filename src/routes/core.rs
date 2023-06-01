use either::*;
use rocket::http::Status;
use rocket::{*, response::status, serde::json::Json};
use crate::path::pth_by_dir;
use crate::reader::{SimpleArchive, list_dir};
use crate::error::SimpleError;
use crate::config::Config;

#[get("/list_dir/<dirname>")]
pub fn listing_by_dirname(dirname: &str, config: &State<Config>) -> status::Custom<Either<Json<Vec<SimpleArchive>>, Json<SimpleError>>> {
    let path = pth_by_dir(dirname, config);

    match list_dir(path.as_path()) {
        Ok(files_list) => {
            let mut result: Vec<SimpleArchive> = Vec::new();
            for archive in files_list {
                result.push(archive.to_simple_archive());
            }
            
            status::Custom(Status::Ok, Left(Json(result)))
        },
        Err(err) => {
            status::Custom(Status::BadRequest, Right(Json(SimpleError::new(err.to_string()))))
        }
    }
}