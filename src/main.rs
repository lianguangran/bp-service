#[macro_use]
extern crate rocket;
use crate::db::BpRecordConn;
use rocket::launch;

pub mod api;
pub mod db;
pub mod error;
pub mod model;
pub mod schema;
pub mod util;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().unwrap();

    rocket::build()
        .attach(BpRecordConn::fairing())
        .mount("/api", api::routes())
        .mount("/api/user", api::user::routes())
        .mount("/api/member", api::member::routes())
        .mount("/api/record", api::record::routes())
}
