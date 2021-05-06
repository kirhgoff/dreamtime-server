use rocket::{http::Status, request, request::FromRequest, request::Outcome, Request, State};
use rocket::outcome::{try_outcome, Outcome::*, IntoOutcome};

#[derive(Debug, Default)]
pub struct ApiKey(pub String);

#[derive(Debug)]
pub enum ApiKeyError {
    MissingKey,
    InvalidKey,
}

#[derive(Debug)]
pub struct Auth(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let original = request.rocket().state::<ApiKey>();
        let provided = request.headers().get_one("X-Api-Key");
        match (original, provided) {
            (Some(original), Some(provided)) if original.0 == provided => {
                Outcome::Success(Auth(String::from("authorized")))
            },
            _ => Outcome::Failure((Status::Unauthorized, ApiKeyError::MissingKey)),
        }
    }
}
