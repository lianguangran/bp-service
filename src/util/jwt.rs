use crate::error::api::ApiError;
use crate::error::auth::AuthError;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use lazy_static::lazy_static;
use rocket::form::{FromFormField, ValueField};
use rocket::request::{FromParam, FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

lazy_static! {
    pub static ref JWT_EXPIRE: u64 = {
        env::var("JWT_EXPIRE")
            .unwrap_or_else(|_| "3600".to_owned())
            .parse::<u64>()
            .unwrap()
    };
    pub static ref KEYS: Keys = {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Keys::new(secret.as_bytes())
    };
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let exp = SystemTime::now() + Duration::from_secs(*JWT_EXPIRE);
        let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        Claims { sub, exp }
    }
}

#[derive(Debug)]
pub struct Uid(pub Uuid);

impl From<Uid> for Uuid {
    fn from(Uid(uuid): Uid) -> Self {
        uuid
    }
}

impl<'a> FromFormField<'a> for Uid {
    fn from_value(field: ValueField<'a>) -> rocket::form::Result<'a, Self> {
        Uuid::parse_str(field.value)
            .map(|uuid| Uid(uuid))
            .map_err(|_| rocket::form::Error::validation("Invalid Uuid").into())
    }
}

impl<'a> FromParam<'a> for Uid {
    type Error = ApiError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param)
            .map(|uuid| Uid(uuid))
            .map_err(|_| ApiError::BadRequest(String::from("Invalid Uuid")))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Uid {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(header) = request.headers().get_one("Authorization") {
            if let Some(token) = header.strip_prefix("Bearer ") {
                let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default());
                if let Ok(token_data) = token_data {
                    let user_id = token_data.claims.sub.parse::<Uuid>();
                    if let Ok(uuid) = user_id {
                        Outcome::Success(Uid(uuid))
                    } else {
                        Outcome::Error(AuthError::WrongCredentials.into())
                    }
                } else {
                    Outcome::Error(AuthError::InvalidToken.into())
                }
            } else {
                Outcome::Error(AuthError::WrongCredentials.into())
            }
        } else {
            Outcome::Error(AuthError::MissingCredentials.into())
        }
    }
}
