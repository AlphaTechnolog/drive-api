use either::*;
use rocket::http::Status;
use rocket::{*, response::status, serde::json::Json};
use rocket::data::{Data, ToByteUnit};
use crate::reader::SimpleArchive;
use crate::error::SimpleError;
use crate::log::SimpleMessage;
use crate::config::Config;
use crate::controllers::fs::Fs;
use crate::path::pth_by_dir;

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

#[post("/upload/<filename>", data = "<file>")]
pub async fn upload(
    filename: String,
    file: Data<'_>,
    config: &State<Config>
) -> status::Custom<Result<Json<SimpleMessage>, Json<SimpleError>>> {
    let path = pth_by_dir(&filename, &config);
    let path_str = path.to_string_lossy().to_string();

    if let Err(err) = file.open(128.kibibytes()).into_file(path).await {
        return status::Custom(
            Status::InternalServerError,
            Err(Json(SimpleError::new(format!("Cannot upload file {}: {}", filename, err.to_string()))))
        );
    }

    status::Custom(
        Status::Ok,
        Ok(Json(SimpleMessage::new(format!("File {} uploaded successfully at {}", filename, path_str))))
    )
}