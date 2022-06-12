use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "adding a new subscriber",
    skip(form, db_pool),
    fields(
        subsriber_email = %form.email,
        subsriber_name = %form.name
    )
)]
pub async fn subsribes(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subcriber(&db_pool, &form).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "saving new subscriber details in the database",
    skip(form, db_pool)
)]
async fn insert_subcriber(db_pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to save subscriber details in the database: {:?}", e);
        e
    })?;

    Ok(())
}
