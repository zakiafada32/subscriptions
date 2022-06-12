use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subsribes(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "adding a new subscriber",
        %request_id,
        subsriber_name = %form.name,
        subsriber_email = %form.email
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            tracing::error!("Failed to save subscriber details in the database: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
