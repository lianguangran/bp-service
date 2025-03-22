use crate::db::user::{NewUser, UserQuery, Users};
use crate::error::api::ApiError;
use crate::error::auth::AuthError;
use crate::model::auth::{AuthBody, WxUser};
use crate::util::jwt::{Claims, KEYS};
use crate::BpRecordConn;
use jsonwebtoken::{encode, Header};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use serde_json::Value;
use std::env;

pub mod member;
pub mod user;
pub mod record;

pub fn routes() -> Vec<rocket::Route> {
    routes![login]
}

#[derive(Deserialize)]
struct LoginPayload {
    code: String,
}

#[post("/login", data = "<payload>")]
async fn login(
    conn: BpRecordConn,
    payload: Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
    let wx_user = wx_login(payload.into_inner().code).await?;

    let user_query = UserQuery {
        openid: Option::from(wx_user.openid.clone()),
        session_key: None,
    };
    let user = Users::select(&conn, Option::from(user_query)).await;
    let user = match user {
        Ok(user) => {
            let new_user = NewUser {
                openid: wx_user.openid,
                session_key: wx_user.session_key,
            };
            if user.is_empty() {
                Users::insert(&conn, new_user).await?
            } else {
                if let Some(user) = user.first() {
                    Users::update(&conn, user.id, new_user).await?
                } else {
                    return Err(AuthError::MissingCredentials.into());
                }
            }
        }
        Err(err) => return Err(ApiError::from(err))
    };

    let claims = Claims::new(user.id.to_string());
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    info!("token: {:?}", token);
    Ok(Json(AuthBody::new(token, claims.exp)))
}

async fn wx_login(code: String) -> Result<WxUser, ApiError> {
    if code.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");
    let code2session_url = format!(
        "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
        app_id, app_secret, code
    );
    let resp = reqwest::get(code2session_url)
        .await
        .map_err(|_| AuthError::WrongCredentials)?
        .json::<Value>()
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    info!("wx_login code: {}, resp: {:?}", code, resp);

    let wx_user =
        serde_json::from_value::<WxUser>(resp).map_err(|_| AuthError::WrongCredentials)?;

    if wx_user.openid.is_empty() {
        Err(AuthError::WrongCredentials.into())
    } else {
        Ok(wx_user)
    }
}
