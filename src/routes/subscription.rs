use actix_web::{web, HttpResponse};
use actix_web::web::Form;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// return a 200 response

pub async fn subscribe(_form : web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}