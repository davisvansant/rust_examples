use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::middleware::Logger;
use env_logger::Env;
use redis::AsyncCommands;
// use redis::Commands;
// use redis::FromRedisValue;
use std::collections::HashSet;

struct Release {
    name: String,
    release_date: String,
}

async fn albums(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Albums!")
}

async fn eps(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("EPs!")
}

async fn do_set_albums() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    // let () = con.zadd_multiple("albums", &[("flight", 2000), ("fuel", 1997), ("caution", 2002)]).await?;
    // let () = con.zadd("albums", "flight", "2000").await?;
    let one = Release { name: "Fuel for the Hate Game".to_string(), release_date: "02.28.1997".to_string() };
    let two = Release { name: "Forever and Counting".to_string(), release_date: "10.28.1997".to_string() };
    let three = Release { name: "No Division".to_string(), release_date: "08.10.1999".to_string() };
    let four = Release { name: "A Flight and a Crash".to_string(), release_date: "06.05.2001".to_string() };
    let five = Release { name: "Caution".to_string(), release_date: "10.08.2002".to_string() };
    let six = Release { name: "The New What Next".to_string(), release_date: "09.21.2004".to_string() };
    let seven = Release { name: "Exister".to_string(), release_date: "05.15.2012".to_string() };
    let eight = Release { name: "Light It Up".to_string(), release_date: "09.15.2017".to_string() };

    con.set(&one.name, one.release_date).await?;
    con.set(&two.name, two.release_date).await?;
    con.set(&three.name, three.release_date).await?;
    con.set(&four.name, four.release_date).await?;
    con.set(&five.name, five.release_date).await?;
    con.set(&six.name, six.release_date).await?;
    con.set(&seven.name, seven.release_date).await?;
    con.set(&eight.name, eight.release_date).await?;

    con.sadd("albums", &one.name).await?;
    con.sadd("albums", &two.name).await?;
    con.sadd("albums", &three.name).await?;
    con.sadd("albums", &four.name).await?;
    con.sadd("albums", &five.name).await?;
    con.sadd("albums", &six.name).await?;
    con.sadd("albums", &seven.name).await?;
    con.sadd("albums", &eight.name).await?;

    let members: HashSet<String> = con.smembers("albums").await?;

    for k in members {
        let v: String = con.get(&k).await?;
        println!("{} - {}", k, v);
    }

    Ok(())
}

async fn do_set_eps() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    let one = Release { name: "(Push For Coin)".to_string(), release_date: "1995".to_string() };
    let two = Release { name: "You Can Take the Boy Out Of Brandenton".to_string(), release_date: "1996".to_string() };
    let three = Release { name: "Alachua".to_string(), release_date: "1997".to_string() };
    let four = Release { name: "Where We Belong / Moonpies For Misfits".to_string(), release_date: "1999".to_string() };
    let five = Release { name: "Moments Pass / Another Way".to_string(), release_date: "1999".to_string() };
    let six = Release { name: "Shake Up The Shadows".to_string(), release_date: "2019".to_string() };

    con.set(&one.name, one.release_date).await?;
    con.set(&two.name, two.release_date).await?;
    con.set(&three.name, three.release_date).await?;
    con.set(&four.name, four.release_date).await?;
    con.set(&five.name, five.release_date).await?;
    con.set(&six.name, six.release_date).await?;

    con.sadd("eps", &one.name).await?;
    con.sadd("eps", &two.name).await?;
    con.sadd("eps", &three.name).await?;
    con.sadd("eps", &four.name).await?;
    con.sadd("eps", &five.name).await?;
    con.sadd("eps", &six.name).await?;

    let members: HashSet<String> = con.smembers("eps").await?;

    for k in members {
        let v: String = con.get(&k).await?;
        println!("{} - {}", k, v);
    }

    Ok(())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    do_set_albums().await.unwrap();
    do_set_eps().await.unwrap();

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
