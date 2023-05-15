use crate::domain::{NewSubscriber, SubscriberName};
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
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let sub = NewSubscriber {
        email: form.0.email,
        name,
    };
    match insert_subscriber(&pool, &sub).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving subscriber details in database", skip(pool, new_sub))]
pub async fn insert_subscriber(pool: &PgPool, new_sub: &NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        new_sub.email,
        new_sub.name.as_ref(),
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
