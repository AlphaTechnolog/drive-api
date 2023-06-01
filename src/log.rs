use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleMessage {
	ok: bool,
	message: String
}

impl SimpleMessage {
	pub fn new(message: String) -> Self {
		Self {
			ok: true,
			message
		}
	}
}