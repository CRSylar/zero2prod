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

pub async fn subscribe(form: web::Form<Formdata>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute Query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
