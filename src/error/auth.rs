use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use std::io::Cursor;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for (Status, AuthError) {
    fn from(auth_error: AuthError) -> (Status, AuthError) {
        match auth_error {
            AuthError::WrongCredentials => (Status::Unauthorized, AuthError::WrongCredentials),
            AuthError::MissingCredentials => (Status::Unauthorized, AuthError::MissingCredentials),
            AuthError::TokenCreation => (Status::InternalServerError, AuthError::TokenCreation),
            AuthError::InvalidToken => (Status::Unauthorized, AuthError::InvalidToken),
        }
    }
}

impl<'r> Responder<'r, 'static> for AuthError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let (status, message) = match self {
            AuthError::WrongCredentials => (Status::Unauthorized, "Wrong credentials"),
            AuthError::MissingCredentials => (Status::Unauthorized, "Missing credentials"),
            AuthError::TokenCreation => (Status::InternalServerError, "Token creation error"),
            AuthError::InvalidToken => (Status::Unauthorized, "Invalid token"),
        };
        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(message.len(), Cursor::new(message))
            .ok()
    }
}
