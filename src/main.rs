#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::{get, post, launch, routes};
use diesel::prelude::*;
use crate::models::{Newsletter, NewNewsletter};
use crate::schema::newsletters::dsl::*;
use crate::schema::newsletters;


mod schema;
mod models;

use rocket_sync_db_pools::{database, diesel};

#[database("newsletter_db")]
pub struct DbConn(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Welcome to Newsletter Manager!"
}

#[get("/newsletters")]
async fn get_newsletters(conn: DbConn) -> Json<Vec<Newsletter>> {
    let results = conn.run(|c| {
        newsletters
            .order(received_at.desc())
            .limit(50)
            .load::<Newsletter>(c)
    }).await;

    match results {
        Ok(newsletter_list) => Json(newsletter_list),
        Err(_) => Json(vec![]), // Return empty array on error
    }
}

#[post("/newsletters/sample")]
async fn create_sample_data(conn: DbConn) -> Json<&'static str> {
    let sample_newsletters = vec![
        NewNewsletter {
            title: "Weekly Tech Newsletter".to_string(),
            sender_email: "tech@example.com".to_string(),
            sender_name: Some("Tech Weekly".to_string()),
            subject: "This Week in Technology".to_string(),
            content: "Here are the top tech stories this week...".to_string(),
            newsletter_type: Some("technology".to_string()),
        },
        NewNewsletter {
            title: "Marketing Insights".to_string(),
            sender_email: "marketing@business.com".to_string(),
            sender_name: Some("Business Insider".to_string()),
            subject: "5 Marketing Trends You Can't Ignore".to_string(),
            content: "Discover the marketing strategies that are working...".to_string(),
            newsletter_type: Some("business".to_string()),
        },
        NewNewsletter {
            title: "Daily News Digest".to_string(),
            sender_email: "news@dailydigest.com".to_string(),
            sender_name: Some("Daily Digest".to_string()),
            subject: "Your Daily News Summary".to_string(),
            content: "Top stories from around the world today...".to_string(),
            newsletter_type: Some("news".to_string()),
        },
    ];

    let result = conn.run(move |c| {
        diesel::insert_into(newsletters::table)
            .values(&sample_newsletters)
            .execute(c)
    }).await;

    match result {
        Ok(_) => Json("Sample newsletters created successfully!"),
        Err(_) => Json("Error creating sample data"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index, get_newsletters, create_sample_data])
}

