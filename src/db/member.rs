use crate::db::user::Users;
use crate::db::BpRecordConn;
use crate::error::api::ApiError;
use crate::schema::{members, user_member};
use crate::util::serde_time_format;
use anyhow::Error;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Identifiable, Queryable, Selectable, Associations)]
#[diesel(
    table_name = user_member,
    primary_key(user_id, member_id),
    belongs_to(Users, foreign_key = user_id),
    belongs_to(Members, foreign_key = member_id),
    check_for_backend(diesel::pg::Pg),
)]
pub struct UserMember {
    pub user_id: Uuid,
    pub member_id: Uuid,
    pub is_default: bool,
    #[serde(with = "serde_time_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "serde_time_format")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Identifiable, Queryable, Selectable)]
#[diesel(
    table_name = crate::schema::members,
    primary_key(id),
    check_for_backend(diesel::pg::Pg),
)]
pub struct Members {
    pub id: Uuid,
    pub name: String,
    pub memo: Option<String>,
    #[serde(with = "serde_time_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "serde_time_format")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromForm)]
pub struct MemberQuery {
    #[field(name = "name")]
    pub name: Option<String>,
    #[field(name = "memo")]
    pub memo: Option<String>,
}

#[derive(Deserialize)]
pub struct NewMember {
    pub name: String,
    pub memo: Option<String>,
}

impl Members {
    pub async fn check_user(
        conn: &BpRecordConn,
        user_id: Uuid,
        member_id: Uuid,
    ) -> Result<UserMember, ApiError> {
        let user_member = conn
            .run(move |c| {
                user_member::table
                    .find((user_id, member_id))
                    .get_result::<UserMember>(c)
            })
            .await
            .map_err(|_| ApiError::BadRequest(String::from("用户与成员信息不匹配")))?;
        Ok(user_member)
    }

    pub async fn detail(conn: &BpRecordConn, id: Uuid) -> Result<Members, ApiError> {
        let member = conn
            .run(move |c| members::table.find(id).get_result::<Members>(c))
            .await?;
        Ok(member)
    }

    pub async fn select(
        conn: &BpRecordConn,
        query: Option<MemberQuery>,
    ) -> Result<Vec<Members>, ApiError> {
        let mut member_query = members::table.into_boxed();
        if let Some(query) = query {
            if let Some(name) = query.name {
                member_query = member_query.filter(members::name.eq(name));
            }
            if let Some(memo) = query.memo {
                member_query = member_query.filter(members::memo.like(format!("%{}%", memo)));
            }
        }
        let member_list = conn.run(|c| member_query.get_results::<Members>(c)).await?;
        Ok(member_list)
    }

    pub async fn insert(
        conn: &BpRecordConn,
        user_id: Uuid,
        new_member: NewMember,
    ) -> Result<Members, ApiError> {
        let member = conn
            .run(move |c| {
                c.transaction(|x| {
                    let member = diesel::insert_into(members::table)
                        .values((
                            members::name.eq(new_member.name),
                            members::memo.eq(new_member.memo),
                        ))
                        .returning(Members::as_returning())
                        .get_result::<Members>(x)
                        .map_err(|e| ApiError::Internal(Error::from(e)));
                    match member {
                        Ok(member) => {
                            diesel::insert_into(user_member::table)
                                .values((
                                    user_member::user_id.eq(user_id),
                                    user_member::member_id.eq(member.id),
                                ))
                                .get_result::<UserMember>(x)
                                .unwrap();
                            Ok(member)
                        }
                        Err(e) => Err(e),
                    }
                })
            })
            .await?;
        Ok(member)
    }

    pub async fn update(
        conn: &BpRecordConn,
        member_id: Uuid,
        new_member: NewMember,
    ) -> Result<Members, ApiError> {
        let member = conn
            .run(move |c| {
                diesel::update(members::table.find(member_id))
                    .set((
                        members::name.eq(new_member.name),
                        members::memo.eq(new_member.memo),
                        members::updated_at.eq(diesel::dsl::now),
                    ))
                    .returning(Members::as_returning())
                    .get_result::<Members>(c)
            })
            .await?;
        Ok(member)
    }

    pub async fn delete(conn: &BpRecordConn, member_id: Uuid) -> Result<usize, ApiError> {
        let result = conn
            .run(move |c| diesel::delete(members::table.find(member_id)).execute(c))
            .await?;
        Ok(result)
    }
}

impl UserMember {
    pub async fn get_user_members(
        conn: &BpRecordConn,
        user_id: Uuid,
    ) -> Result<Vec<Members>, ApiError> {
        let member_list = conn
            .run(move |c| {
                members::table
                    .inner_join(user_member::table)
                    .filter(user_member::user_id.eq(user_id))
                    .select(Members::as_select())
                    .get_results::<Members>(c)
            })
            .await?;
        Ok(member_list)
    }
}
