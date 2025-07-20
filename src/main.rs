#[macro_use] extern crate rocket;

mod schema;
mod models;

use rocket::serde::json::Json;
use rocket::{State, get, launch, routes};
use rocket_sync_db_pools::{database, diesel};

#[database("newsletter_db")]
pub struct DbConn(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Welcome to Newsletter Manager!"
}

#[get("/newsletters")]
fn get_newsletters() -> Json<Vec<String>> {
    // For now, return a simple test response
    Json(vec![
        "Newsletter 1".to_string(),
        "Newsletter 2".to_string(),
        "Newsletter 3".to_string(),
    ])
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index, get_newsletters])
}

