use crate::db::member::UserMember;
use crate::db::member::{MemberQuery, Members, NewMember};
use crate::db::BpRecordConn;
use crate::error::api::ApiError;
use crate::util::jwt::Uid;
use rocket::routes;
use rocket::serde::json::Json;

pub fn routes() -> Vec<rocket::Route> {
    routes![members, add_member, edit_member, delete_member, detail]
}

#[get("/all")]
async fn members(conn: BpRecordConn, user_id: Uid) -> Result<Json<Vec<Members>>, ApiError> {
    let member_list = UserMember::get_user_members(&conn, user_id.into()).await?;
    Ok(Json(member_list))
}

#[post("/", data = "<new_member>")]
async fn add_member(
    conn: BpRecordConn,
    user_id: Uid,
    new_member: Json<NewMember>,
) -> Result<Json<Members>, ApiError> {
    let member = Members::insert(&conn, user_id.into(), new_member.into_inner()).await?;
    Ok(Json(member))
}

#[put("/<member_id>", data = "<new_member>")]
async fn edit_member(
    conn: BpRecordConn,
    user_id: Uid,
    member_id: Uid,
    new_member: Json<NewMember>,
) -> Result<Json<Members>, ApiError> {
    let user_member = Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    let member = Members::update(&conn, user_member.member_id, new_member.into_inner()).await?;
    Ok(Json(member))
}

#[delete("/<member_id>")]
async fn delete_member(conn: BpRecordConn, user_id: Uid, member_id: Uid) -> Result<(), ApiError> {
    let user_member = Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    Members::delete(&conn, user_member.member_id).await?;
    Ok(())
}

#[get("/<member_id>")]
async fn detail(
    conn: BpRecordConn,
    user_id: Uid,
    member_id: Uid,
) -> Result<Json<Members>, ApiError> {
    let user_member = Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    let member = Members::detail(&conn, user_member.member_id).await?;
    Ok(Json(member))
}

#[allow(unused)]
#[get("/?<query..>")]
async fn all(
    conn: BpRecordConn,
    query: Option<MemberQuery>,
) -> Result<Json<Vec<Members>>, ApiError> {
    let member_list = Members::select(&conn, query).await?;
    Ok(Json(member_list))
}
