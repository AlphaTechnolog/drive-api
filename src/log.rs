use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleMessage {
	message: String
}

impl SimpleMessage {
	pub fn new(message: String) -> Self {
		Self {
			message
		}
	}
}