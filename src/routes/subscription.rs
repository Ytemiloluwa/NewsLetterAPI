use actix_web::{web, HttpResponse};
use actix_web::web::Form;
use chrono::Utc;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// return a 200 response

pub async fn subscribe(form : web::Form<FormData>,
                       // retrieving a connection from the application state
connection: web::Data<PgConnection>, ) -> HttpResponse {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)

    "#, Uuid::new_v4(), form.email, form.name, Utc::now())
        // use get_ref to get an immutable reference to the PgConnection
        // wrapped by 'web::Data'
        .execute(connection.get_ref())
        .await;
    HttpResponse::Ok().finish()
}