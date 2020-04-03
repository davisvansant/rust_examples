// use actix_web::*;
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use env_logger::Env;
// use juniper::*;
use juniper::http::graphiql::graphiql_source;
use juniper::http::playground::playground_source;
use std::sync::Arc;
// use serde_json::*;
use serde_json::{Result, Value, Error as serde_json_error};
use juniper::http::GraphQLRequest;

mod schema;

use crate::schema::create_schema;
use crate::schema::Schema;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn playground() -> HttpResponse {
    let html = playground_source("/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> HttpResponse {
    let res = data.execute(&st, &());
    let user = serde_json::to_string(&res).ok().unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(user)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
            // .route("/graphql", web::post().to(graphql))
            // .route("/graphiql", web::get().to(graphiql))
            // .route("/playground", web::get().to(playground))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
