use actix_web::{web, App, Responder, HttpServer, get, HttpResponse};
use actix_web::middleware::Logger;
use env_logger::Env;

#[get("/albums")]
async fn albums() -> impl Responder {
    HttpResponse::Ok().body("Albums!")
}

#[get("/eps")]
async fn eps() -> impl Responder {
    HttpResponse::Ok().body("EPs!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(albums)
            .service(eps)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
