use crate::db::BpRecordConn;
use crate::error::api::ApiError;
use crate::schema::users;
use crate::util::serde_time_format;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Identifiable, Queryable, Selectable)]
#[diesel(
    table_name = crate::schema::users,
    primary_key(id),
    check_for_backend(diesel::pg::Pg),
)]
pub struct Users {
    pub id: Uuid,
    pub openid: String,
    pub session_key: String,
    #[serde(with = "serde_time_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "serde_time_format")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromForm)]
pub struct UserQuery {
    #[field(name = "openid")]
    pub openid: Option<String>,
    #[field(name = "session_key")]
    pub session_key: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(
    table_name = crate::schema::users,
    check_for_backend(diesel::pg::Pg),
)]
pub struct NewUser {
    pub openid: String,
    pub session_key: String,
}

impl Users {
    pub async fn detail(conn: &BpRecordConn, id: Uuid) -> Result<Users, ApiError> {
        let user = conn
            .run(move |c| users::table.find(id).get_result::<Users>(c))
            .await?;
        Ok(user)
    }

    pub async fn select(
        conn: &BpRecordConn,
        query: Option<UserQuery>,
    ) -> Result<Vec<Users>, ApiError> {
        let mut user_query = users::table.into_boxed();
        if let Some(query) = query {
            if let Some(openid) = query.openid {
                user_query = user_query.filter(users::openid.eq(openid));
            }
            if let Some(session_key) = query.session_key {
                user_query = user_query.filter(users::session_key.eq(session_key));
            }
        }
        let users_list = conn.run(|c| user_query.get_results::<Users>(c)).await?;
        Ok(users_list)
    }

    pub async fn insert(conn: &BpRecordConn, new_user: NewUser) -> Result<Users, ApiError> {
        let user = conn
            .run(move |c| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .get_result::<Users>(c)
            })
            .await?;
        Ok(user)
    }

    pub async fn update(
        conn: &BpRecordConn,
        id: Uuid,
        new_user: NewUser,
    ) -> Result<Users, ApiError> {
        let user = conn
            .run(move |c| {
                diesel::update(users::table.find(id))
                    .set((
                        users::openid.eq(new_user.openid),
                        users::session_key.eq(new_user.session_key),
                        users::updated_at.eq(diesel::dsl::now),
                    ))
                    .get_result::<Users>(c)
            })
            .await?;
        Ok(user)
    }

    pub async fn delete(conn: &BpRecordConn, id: Uuid) -> Result<usize, ApiError> {
        let result = conn
            .run(move |c| diesel::delete(users::table.find(id)).execute(c))
            .await?;
        Ok(result)
    }
}
