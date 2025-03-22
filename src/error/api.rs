use crate::error::auth::AuthError;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use std::io::Cursor;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Auth(AuthError),
    BadRequest(String),
    Internal(anyhow::Error),
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> Result<Response<'static>, Status> {
        match self {
            ApiError::NotFound => Response::build()
                .header(ContentType::JSON)
                .status(Status::NotFound)
                .ok(),
            ApiError::Auth(err) => err.respond_to(request),
            ApiError::BadRequest(err) => Response::build()
                .header(ContentType::JSON)
                .status(Status::BadRequest)
                .sized_body(err.len(), Cursor::new(err))
                .ok(),
            ApiError::Internal(err) => Response::build()
                .header(ContentType::JSON)
                .status(Status::InternalServerError)
                .sized_body(err.to_string().len(), Cursor::new(err.to_string()))
                .ok(),
        }
    }
}
