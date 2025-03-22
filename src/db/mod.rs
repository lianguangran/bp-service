use rocket_sync_db_pools::database;

pub mod user;
pub mod member;
pub mod record;

#[database("bp-record")]
pub struct BpRecordConn(diesel::PgConnection);
