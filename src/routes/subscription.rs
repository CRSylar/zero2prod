use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Formdata {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<Formdata>) -> HttpResponse {
    return HttpResponse::Ok().finish();
}
