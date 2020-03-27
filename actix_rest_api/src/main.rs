use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::middleware::Logger;
use env_logger::Env;

async fn albums(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Albums!")
}

async fn eps(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("EPs!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/albums", web::get().to(albums))
            .route("/eps", web::get().to(eps))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn unit_handler_albums() {
        let req = test::TestRequest::default().to_http_request();
        let resp = albums(req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn unit_handler_eps() {
        let req = test::TestRequest::default().to_http_request();
        let resp = eps(req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn integration_handler_albums() {
        let mut app = test::init_service(App::new().route("/albums", web::get().to(albums))).await;
        let req = test::TestRequest::get().uri("/albums").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_handler_eps() {
        let mut app = test::init_service(App::new().route("/eps", web::get().to(albums))).await;
        let req = test::TestRequest::get().uri("/eps").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
