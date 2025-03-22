use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

impl AuthBody {
    pub fn new(access_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            token_type: String::from("Bearer"),
            expires_in,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct WxUser {
    pub openid: String,
    pub session_key: String,
}
