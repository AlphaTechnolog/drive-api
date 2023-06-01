use either::*;
use rocket::http::Status;
use rocket::{*, response::status, serde::json::Json};
use crate::reader::SimpleArchive;
use crate::error::SimpleError;
use crate::log::SimpleMessage;
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

#[get("/read_file/<filename>")]
pub fn read_file(filename: String, config: &State<Config>) -> status::Custom<Either<String, Json<SimpleError>>> {
    let fs = Fs {
        dirname: filename,
        config
    };

    match fs.read_file() {
        Ok(content) => {
            status::Custom(Status::Ok, Left(content))
        },
        Err(error) => {
            status::Custom(
                Status::BadRequest,
                Right(Json(SimpleError::new(error.to_string())))
            )
        }
    }
}

#[delete("/remove/<dirname>")]
pub fn remove(dirname: String, config: &State<Config>) -> status::Custom<Either<Json<SimpleMessage>, Json<SimpleError>>> {
    let fs = Fs {
        dirname: dirname,
        config
    };

    match fs.remove() {
        Ok(message) => {
            status::Custom(
                Status::Ok,
                Left(Json(message))
            )
        },
        Err(error) => {
            status::Custom(
                Status::BadRequest,
                Right(Json(error))
            )
        }
    }
}