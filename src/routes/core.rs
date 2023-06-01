use either::*;
use rocket::http::Status;
use rocket::{*, response::status, serde::json::Json};
use crate::reader::SimpleArchive;
use crate::error::SimpleError;
use crate::config::Config;
use crate::controllers::fs::Fs;

#[get("/list_dir/<dirname>")]
pub fn listing_by_dirname(dirname: String, config: &State<Config>) -> status::Custom<Either<Json<Vec<SimpleArchive>>, Json<SimpleError>>> {
    let fs = Fs {
        dirname,
        config
    };

    match fs.list() {
        Ok(files_list) => {
            status::Custom(Status::Ok, Left(Json(files_list)))
        },
        Err(error) => {
            status::Custom(Status::BadRequest, Right(Json(error)))
        }
    }
}