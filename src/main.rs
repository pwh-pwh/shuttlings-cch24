use actix_web::http::StatusCode;
use actix_web::{get, web, web::ServiceConfig, HttpResponse};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::str::FromStr;

#[derive(Deserialize)]
struct QueryInfo {
    from: String,
    key: String,
}

#[derive(Deserialize)]
struct QueryInfo2 {
    from: String,
    to: String
}

#[get("/")]
async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
async fn find_seek() -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::FOUND)
        .insert_header(("location", "https://www.youtube.com/watch?v=9Gc4QTqslN4"))
        .finish()
}

#[get("/2/dest")]
async fn eg_encryption(info: web::Query<QueryInfo>) -> String {
    let from_vec = info
        .from
        .split('.')
        .map(|s| u8::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let key_vec = info
        .key
        .split('.')
        .map(|s| u8::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let mut result_vec = vec![];
    for i in 0..4 {
        let r = from_vec[i].overflowing_add(key_vec[i]);
        result_vec.push(r.0);
    }
    format!("{}.{}.{}.{}", result_vec[0], result_vec[1],result_vec[2],result_vec[3])
}

#[get("/2/key")]
async fn day2_task2(info: web::Query<QueryInfo2>) -> String {
    let from_vec = info
        .from
        .split('.')
        .map(|s| u8::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let to_vec = info
        .to
        .split('.')
        .map(|s| u8::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let mut result_vec = vec![];
    for i in 0..4 {
        let r = to_vec[i].overflowing_sub(from_vec[i]);
        result_vec.push(r.0);
    }
    format!("{}.{}.{}.{}", result_vec[0], result_vec[1],result_vec[2],result_vec[3])
}

#[get("/2/v6/dest")]
async fn v6_dest(info: web::Query<QueryInfo>) -> String {
    todo!()
}

#[get("/2/v6/key")]
async fn v6_key(info: web::Query<QueryInfo2>) -> String {
    todo!()
}

fn to_ip_v6_vec(s: &str) -> [u16;8] {
    todo!()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird)
            .service(find_seek)
            .service(eg_encryption)
            .service(day2_task2);
    };

    Ok(config.into())
}
