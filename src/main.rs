use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ping!")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/healthz", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
