use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn greet() -> HttpResponse {
    return HttpResponse::Ok().json("Hello World!");
}

async fn health_check() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

#[derive(serde::Deserialize)]
struct Formdata {
    email: String,
    name: String,
}

async fn subscribe(form: web::Form<Formdata>) -> HttpResponse {
    return HttpResponse::Ok().finish();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
