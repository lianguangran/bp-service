use rocket_sync_db_pools::database;

pub mod member;
pub mod record;
pub mod user;

#[database("bp-record")]
pub struct BpRecordConn(diesel::PgConnection);
