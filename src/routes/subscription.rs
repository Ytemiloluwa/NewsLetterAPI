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

pub async fn subscribe(form : web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // generate a random unique identifier
    let request_id = Uuid::new_v4();
    log::info!("request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    log::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)

    "#, Uuid::new_v4(), form.email, form.name, Utc::now())
        // use get_ref to get an immutable reference to the PgConnection
        // wrapped by 'web::Data'
        .execute(pool.get_ref())
        .await {

        Ok(_) => {
            log::info!("request_id {} - New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::info!("request_id {} - Failed to execute query {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}