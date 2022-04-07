use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

#[get("/healthz")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ping!")
}

pub fn run() -> Result<Server, std::io::Error> {
    let server =HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
        .bind(("127.0.0.1", 8080))?
        .run();

    Ok(server)
}