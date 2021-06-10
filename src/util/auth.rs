use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};
use rocket::outcome::IntoOutcome;

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct User(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, ()> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(User)
            .into_outcome((Status::Unauthorized, ()))
    }
}
