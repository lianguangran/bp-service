use crate::db::BpRecordConn;
use crate::db::member::Members;
use crate::error::api::ApiError;
use crate::schema::{members, records};
use crate::util::serde_time_format;
use chrono::{Months, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

lazy_static! {
    pub static ref RECORD_MONTH: u32 = {
        env::var("RECORD_MONTH")
            .unwrap_or_else(|_| "2".to_owned())
            .parse::<u32>()
            .unwrap()
    };
}

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
                let record_months_ago = Utc::now()
                    .checked_sub_months(Months::new(*RECORD_MONTH))
                    .unwrap()
                    .naive_utc();
                records::table
                    .inner_join(members::table)
                    .filter(members::id.eq(member_id))
                    .filter(records::record_at.ge(record_months_ago))
                    .order((records::record_at.desc(), records::updated_at.desc()))
                    .select(Records::as_select())
                    .get_results::<Records>(c)
            })
            .await?;
        Ok(record_list)
    }

    pub async fn detail(conn: &BpRecordConn, record_id: Uuid) -> Result<Records, ApiError> {
        let record = conn
            .run(move |c| records::table.find(record_id).get_result::<Records>(c))
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
                c.transaction(|x| {
                    let record = diesel::insert_into(records::table)
                        .values((
                            records::member_id.eq(member_id),
                            records::systolic.eq(new_record.systolic),
                            records::diastolic.eq(new_record.diastolic),
                            records::bmp.eq(new_record.bmp),
                            records::record_at.eq(new_record.record_at),
                        ))
                        .get_result::<Records>(x);
                    if record.is_ok() {
                        diesel::update(members::table.find(member_id))
                            .set(members::updated_at.eq(diesel::dsl::now))
                            .get_result::<Members>(x)?;
                    }
                    record
                })
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
                c.transaction(|x| {
                    let record = diesel::update(records::table.find(record_id))
                        .set((
                            records::member_id.eq(member_id),
                            records::systolic.eq(new_record.systolic),
                            records::diastolic.eq(new_record.diastolic),
                            records::bmp.eq(new_record.bmp),
                            records::record_at.eq(new_record.record_at),
                            records::updated_at.eq(diesel::dsl::now),
                        ))
                        .get_result::<Records>(x);
                    if record.is_ok() {
                        diesel::update(members::table.find(member_id))
                            .set(members::updated_at.eq(diesel::dsl::now))
                            .get_result::<Members>(x)?;
                    }
                    record
                })
            })
            .await?;
        Ok(record)
    }

    pub async fn delete(conn: &BpRecordConn, record_id: Uuid) -> Result<usize, ApiError> {
        let num = conn
            .run(move |c| {
                c.transaction(|x| {
                    let record = records::table.find(record_id).get_result::<Records>(x);
                    if let Ok(record) = record {
                        diesel::update(members::table.find(record.member_id))
                            .set(members::updated_at.eq(diesel::dsl::now))
                            .get_result::<Members>(x)?;
                    }
                    diesel::delete(records::table.find(record_id)).execute(x)
                })
            })
            .await?;
        Ok(num)
    }
}
