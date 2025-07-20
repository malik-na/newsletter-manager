use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::newsletters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Newsletter {
    pub id: i32,
    pub title: String,
    pub sender_email: String,
    pub sender_name: Option<String>,
    pub subject: String,
    pub content: String,
    pub newsletter_type: String,
    pub is_read: bool,
    pub is_favorite: bool,
    pub importance_score: f32,
    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::newsletters)]
pub struct NewNewsletter {
    pub title: String,
    pub sender_email: String,
    pub sender_name: Option<String>,
    pub subject: String,
    pub content: String,
    pub newsletter_type: Option<String>,
}

