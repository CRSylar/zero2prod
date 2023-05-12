use actix_web::{web, HttpResponse};
use sqlx::types::chrono::Utc;
use sqlx::types::uuid;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Formdata {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new Subscriber",
    skip(form, pool),
    fields(
        subscriber_name = %form.name,
        subscriber_email = %form.email
    )
)]
pub async fn subscribe(form: web::Form<Formdata>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving subscriber details in database", skip(pool, form))]
pub async fn insert_subscriber(pool: &PgPool, form: &Formdata) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute Query: {:?}", e);
        e
    })?;

    Ok(())
}
