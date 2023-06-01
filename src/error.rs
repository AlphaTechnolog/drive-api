use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleError {
    message: String
}

impl SimpleError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}