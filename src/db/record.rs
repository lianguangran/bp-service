use crate::db::BpRecordConn;
use crate::error::api::ApiError;
use crate::schema::{members, records};
use crate::util::serde_time_format;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Identifiable, Queryable, Selectable)]
#[diesel(
    table_name = crate::schema::records,
    primary_key(id),
    check_for_backend(diesel::pg::Pg),
)]
pub struct Records {
    pub id: Uuid,
    pub member_id: Uuid,
    pub systolic: i32,
    pub diastolic: i32,
    pub bmp: i32,
    #[serde(with = "serde_time_format")]
    pub record_at: NaiveDateTime,
    #[serde(with = "serde_time_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "serde_time_format")]
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewRecord {
    pub systolic: i32,
    pub diastolic: i32,
    pub bmp: i32,
    #[serde(with = "serde_time_format")]
    pub record_at: NaiveDateTime,
}

impl Records {
    pub async fn get_member_record(
        conn: &BpRecordConn,
        member_id: Uuid,
    ) -> Result<Vec<Records>, ApiError> {
        let record_list = conn
            .run(move |c| {
                records::table
                    .inner_join(members::table)
                    .filter(members::id.eq(member_id))
                    .select(Records::as_select())
                    .load::<Records>(c)
            })
            .await?;
        Ok(record_list)
    }

    pub async fn detail(conn: &BpRecordConn, record_id: Uuid) -> Result<Records, ApiError> {
        let record = conn
            .run(move |c| {
                records::table
                    .find(record_id)
                    .select(Records::as_select())
                    .get_result::<Records>(c)
            })
            .await?;
        Ok(record)
    }

    pub async fn insert(
        conn: &BpRecordConn,
        member_id: Uuid,
        new_record: NewRecord,
    ) -> Result<Records, ApiError> {
        let record = conn
            .run(move |c| {
                diesel::insert_into(records::table)
                    .values((
                        records::member_id.eq(member_id),
                        records::systolic.eq(new_record.systolic),
                        records::diastolic.eq(new_record.diastolic),
                        records::bmp.eq(new_record.bmp),
                        records::record_at.eq(new_record.record_at),
                    ))
                    .returning(Records::as_returning())
                    .get_result(c)
            })
            .await?;
        Ok(record)
    }

    pub async fn update(
        conn: &BpRecordConn,
        record_id: Uuid,
        member_id: Uuid,
        new_record: NewRecord,
    ) -> Result<Records, ApiError> {
        let record = conn
            .run(move |c| {
                diesel::update(records::table.find(record_id))
                    .set((
                        records::member_id.eq(member_id),
                        records::systolic.eq(new_record.systolic),
                        records::diastolic.eq(new_record.diastolic),
                        records::bmp.eq(new_record.bmp),
                        records::record_at.eq(new_record.record_at),
                        records::updated_at.eq(diesel::dsl::now),
                    ))
                    .returning(Records::as_returning())
                    .get_result(c)
            })
            .await?;
        Ok(record)
    }

    pub async fn delete(conn: &BpRecordConn, record_id: Uuid) -> Result<usize, ApiError> {
        let num = conn
            .run(move |c| diesel::delete(records::table.find(record_id)).execute(c))
            .await?;
        Ok(num)
    }
}
