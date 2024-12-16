mod entry;

use actix_web::http::{header, StatusCode};
use actix_web::{get, post, web, web::ServiceConfig, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::net::Ipv6Addr;
use std::ops::BitXor;
use std::str::FromStr;
use actix_web::http::header::ContentType;
use actix_web::web::{head, Header};
use cargo_manifest::Manifest;
use crate::entry::Config;

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
async fn manifest_api(req_body: String,header: web::Header<header::ContentType>) -> impl Responder {
    println!("header: {}", header);
    println!("req_body: {}", req_body);
    match header.to_string().as_str() {
        "application/toml" => deal_with_toml(&req_body),
        "application/yaml" => deal_with_yaml(&req_body),
        "application/json" => deal_with_json(&req_body),
        _ => HttpResponse::Ok().status(StatusCode::UNSUPPORTED_MEDIA_TYPE).finish()
    }
}

fn deal_with_toml(req_body: &str) -> impl Responder {
    match Manifest::from_str(&req_body) {
        Ok(manifest) => {
            let cf_flag = manifest.package.unwrap().keywords.and_then(|k|k.as_local())
                .map(|k| k.contains(&"Christmas 2024".to_string()))
                .unwrap_or_default();
            if !cf_flag {
                return HttpResponse::Ok().status(StatusCode::BAD_REQUEST).body("Magic keyword not provided");
            }
            if let Ok(config) = toml::from_str::<Config>(&req_body) {
                // check magic word
                let r = config.package.metadata.orders
                    .iter()
                    .filter(|o| o.quantity.is_some())
                    .map(|item| format!("{}: {}", item.item, item.quantity.unwrap()))
                    .collect::<Vec<String>>().join("\n");
                if r.is_empty() {
                    return HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
                }
                HttpResponse::Ok().body(r)
            } else {
                HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
            }
        },
        Err(e) => HttpResponse::Ok().status(StatusCode::BAD_REQUEST).body("Invalid manifest"),
    }
}

fn deal_with_yaml(req_body: &str) -> impl Responder {
    todo!()
}

fn deal_with_json(req_body: &str) -> impl Responder {
    todo!()
}


#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird)
            .service(find_seek)
            .service(eg_encryption)
            .service(day2_task2)
            .service(v6_dest)
            .service(v6_key)
            .service(manifest_api);
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