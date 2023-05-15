use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
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
    let sub = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
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
        new_sub.email.as_ref(),
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

impl TryFrom<Formdata> for NewSubscriber {
    type Error = String;

    fn try_from(value: Formdata) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(Self { name, email })
    }
}
