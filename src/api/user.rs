use crate::db::user::{NewUser, UserQuery, Users};
use crate::db::BpRecordConn;
use crate::error::api::ApiError;
use crate::util::jwt::Uid;
use rocket::routes;
use rocket::serde::json::Json;

pub fn routes() -> Vec<rocket::Route> {
    routes![detail]
}

#[get("/")]
async fn detail(conn: BpRecordConn, id: Uid) -> Result<Json<Users>, ApiError> {
    let user = Users::detail(&conn, id.into()).await?;
    Ok(Json(user))
}

#[allow(unused)]
#[get("/?<query..>")]
async fn all(conn: BpRecordConn, query: Option<UserQuery>) -> Result<Json<Vec<Users>>, ApiError> {
    let user_list = Users::select(&conn, query).await?;
    Ok(Json(user_list))
}

#[allow(unused)]
#[post("/", data = "<new_user>")]
async fn add(conn: BpRecordConn, new_user: Json<NewUser>) -> Result<Json<Users>, ApiError> {
    let user = Users::insert(&conn, new_user.into_inner()).await?;
    Ok(Json(user))
}

#[allow(unused)]
#[put("/", data = "<new_user>")]
async fn modify(
    conn: BpRecordConn,
    id: Uid,
    new_user: Json<NewUser>,
) -> Result<Json<Users>, ApiError> {
    let user = Users::update(&conn, id.into(), new_user.into_inner()).await?;
    Ok(Json(user))
}

#[allow(unused)]
#[delete("/")]
async fn delete(conn: BpRecordConn, id: Uid) -> Result<(), ApiError> {
    Users::delete(&conn, id.into()).await?;
    Ok(())
}
