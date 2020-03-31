use actix_web::*;
use actix_web::middleware::Logger;
use env_logger::Env;
use juniper::*;
use juniper::http::graphiql::graphiql_source;
use juniper::http::playground::playground_source;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphiql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn playground() -> HttpResponse {
    let html = playground_source("/playground");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/graphiql", web::get().to(graphiql))
            .route("/playground", web::get().to(playground))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
