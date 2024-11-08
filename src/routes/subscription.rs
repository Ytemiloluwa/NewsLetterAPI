use actix_web::{web, HttpResponse};
use actix_web::web::Form;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// return a 200 response

pub async fn subscribe(form : web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // generate a random unique identifier
    let request_id = Uuid::new_v4();
    // info_span creates a span at the info level
    let request_span = tracing::info_span!("Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name);
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)

    "#, Uuid::new_v4(), form.email, form.name, Utc::now())
        // use get_ref to get an immutable reference to the PgConnection
        // wrapped by 'web::Data'
        .execute(pool.get_ref())
        // attaching instrumentation
        .instrument(query_span)
        .await
    {

        Ok(_) => {
            tracing::info!("request_id {} - New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}