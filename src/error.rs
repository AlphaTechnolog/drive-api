use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleError {
    error: bool,
    message: String
}

impl SimpleError {
    pub fn new(message: String) -> Self {
        Self {
            error: true,
            message
        }
    }
}