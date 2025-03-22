use crate::db::member::Members;
use crate::db::record::{NewRecord, Records};
use crate::db::BpRecordConn;
use crate::error::api::ApiError;
use crate::util::jwt::Uid;
use rocket::serde::json::Json;

pub fn routes() -> Vec<rocket::Route> {
    routes![records, add_record, edit_record, delete_record, detail]
}

#[get("/<member_id>")]
async fn records(
    conn: BpRecordConn,
    user_id: Uid,
    member_id: Uid,
) -> Result<Json<Vec<Records>>, ApiError> {
    let user_member = Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    let record_list = Records::get_member_record(&conn, user_member.member_id).await?;
    Ok(Json(record_list))
}

#[post("/<member_id>", data = "<new_record>")]
async fn add_record(
    conn: BpRecordConn,
    user_id: Uid,
    member_id: Uid,
    new_record: Json<NewRecord>,
) -> Result<Json<Records>, ApiError> {
    let user_member = Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    let record = Records::insert(&conn, user_member.member_id, new_record.into_inner()).await?;
    Ok(Json(record))
}

#[put("/<member_id>/<record_id>", data = "<new_record>")]
async fn edit_record(
    conn: BpRecordConn,
    user_id: Uid,
    member_id: Uid,
    record_id: Uid,
    new_record: Json<NewRecord>,
) -> Result<Json<Records>, ApiError> {
    let user_member = Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    let record = Records::update(
        &conn,
        record_id.into(),
        user_member.member_id,
        new_record.into_inner(),
    )
    .await?;
    Ok(Json(record))
}

#[delete("/<member_id>/<record_id>")]
async fn delete_record(
    conn: BpRecordConn,
    user_id: Uid,
    member_id: Uid,
    record_id: Uid,
) -> Result<(), ApiError> {
    Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    Records::delete(&conn, record_id.into()).await?;
    Ok(())
}

#[get("/<member_id>/<record_id>")]
async fn detail(
    conn: BpRecordConn,
    user_id: Uid,
    member_id: Uid,
    record_id: Uid,
) -> Result<Json<Records>, ApiError> {
    Members::check_user(&conn, user_id.into(), member_id.into()).await?;
    let record = Records::detail(&conn, record_id.into()).await?;
    Ok(Json(record))
}
