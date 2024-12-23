mod entry;

use crate::entry::Metadata;
use actix_web::http::{header, StatusCode};
use actix_web::{error, get, post, web, web::ServiceConfig, HttpRequest, HttpResponse, Responder};
use cargo_manifest::Manifest;
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::__internals::serde_json;
use shuttlings_cch24::day9::{milk, new_rate_limiter, refill};
use std::net::Ipv6Addr;
use std::ops::BitXor;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use shuttlings_cch24::day12::{board, place, random_board, reset, Board};

#[derive(Deserialize)]
struct QueryInfo {
    from: String,
    key: String,
}

#[derive(Deserialize)]
struct QueryInfo2 {
    from: String,
    to: String,
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
    format!(
        "{}.{}.{}.{}",
        result_vec[0], result_vec[1], result_vec[2], result_vec[3]
    )
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
    format!(
        "{}.{}.{}.{}",
        result_vec[0], result_vec[1], result_vec[2], result_vec[3]
    )
}

#[get("/2/v6/dest")]
async fn v6_dest(info: web::Query<QueryInfo>) -> String {
    let from_vec = to_ip_v6_vec(&info.from);
    let key_vec = to_ip_v6_vec(&info.key);
    let mut result_vec = vec![];
    for i in 0..8 {
        let r = from_vec[i].bitxor(key_vec[i]);
        result_vec.push(r);
    }
    let r = Ipv6Addr::from([
        result_vec[0],
        result_vec[1],
        result_vec[2],
        result_vec[3],
        result_vec[4],
        result_vec[5],
        result_vec[6],
        result_vec[7],
    ])
    .to_string();
    r
}

#[get("/2/v6/key")]
async fn v6_key(info: web::Query<QueryInfo2>) -> String {
    let from_vec = to_ip_v6_vec(&info.from);
    let to_vec = to_ip_v6_vec(&info.to);
    let mut result_vec = vec![];
    for i in 0..8 {
        let r = from_vec[i].bitxor(to_vec[i]);
        result_vec.push(r);
    }
    let r = Ipv6Addr::from([
        result_vec[0],
        result_vec[1],
        result_vec[2],
        result_vec[3],
        result_vec[4],
        result_vec[5],
        result_vec[6],
        result_vec[7],
    ])
    .to_string();
    r
}

pub fn to_ip_v6_vec(s: &str) -> Vec<u16> {
    Ipv6Addr::from_str(s).unwrap().segments().to_vec()
    /*let mut vec = s.split(':').collect::<Vec<&str>>();
    println!("vec: {:?}", vec);
    let flag = vec.iter().all(|item| !item.is_empty());
    if !flag {
        let oth_len = 8 - vec.len() + 1;
        println!("oth_len: {}", oth_len);
        if let Some(index) = vec.iter().position(|&x| x.is_empty()) {
            println!("index: {} is empty", index);
            vec.remove(index);
            for _ in 0..oth_len {
                vec.insert(index, "0");
            }
        }
    }
    vec.iter()
        .map(|item| u16::from_str_radix(item, 16).unwrap())
        .collect::<Vec<u16>>()*/
}

#[post("/5/manifest")]
async fn manifest_api(req: HttpRequest, data: String) -> impl Responder {
    let Ok(Some(package)) = (match req
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
    {
        Some(ct) => match ct {
            "application/toml" => toml::from_str::<Manifest<Metadata>>(&data)
                .map_err(|_| "Invalid TOML Manifest".to_string()),
            "application/json" => serde_json::from_str::<Manifest<Metadata>>(&data)
                .map_err(|_| "Invalid JSON Manifest".to_string()),
            "application/yaml" => serde_yml::from_str::<Manifest<Metadata>>(&data)
                .map_err(|_| "Invalid YAML manifest".to_string()),
            _ => return HttpResponse::UnsupportedMediaType().finish(),
        },
        None => return HttpResponse::UnsupportedMediaType().finish(),
    })
    .map(|m| m.package) else {
        return HttpResponse::BadRequest().body("Invalid manifest");
    };

    // Check for code in keyword
    if !package
        .keywords
        .and_then(|k| k.as_local())
        .map(|k| k.contains(&"Christmas 2024".to_string()))
        .unwrap_or_default()
    {
        return HttpResponse::BadRequest().body("Magic keyword not provided");
    }

    // Process orders
    let Some(orders) = package.metadata.map(|m| {
        m.orders
            .into_iter()
            .filter(|o| o.quantity.is_some())
            .map(|o| format!("{}: {}", o.item, o.quantity.unwrap()))
            .collect::<Vec<String>>()
    }) else {
        return HttpResponse::NoContent().finish();
    };

    // return no content if no valid order
    if orders.is_empty() {
        return HttpResponse::NoContent().finish();
    }

    // Final response
    HttpResponse::Ok().body(orders.join("\n"))
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let limiter = new_rate_limiter();
    let bucket = Arc::new(Mutex::new(limiter));
    let grid = Arc::new(RwLock::new(Board::default()));
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird)
            .service(find_seek)
            .service(eg_encryption)
            .service(day2_task2)
            .service(v6_dest)
            .service(v6_key)
            .app_data(web::Data::new(bucket.clone()))
            .service(milk)
            .service(refill)
            .service(manifest_api)
            .app_data(web::Data::new(grid.clone()))
            .service(board)
            .service(reset)
            .service(place)
            .service(random_board)
            .app_data(web::PathConfig::default().error_handler(|err, _| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().into()).into()
            }));
    };

    Ok(config.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ip_v6_vec() {
        let s = "::1";
        let v = to_ip_v6_vec(&s);
        println!("v: {:?}", v);
        assert_eq!(v.len(), 8);
    }
}
