use actix_web::http::header::{ContentType, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use leaky_bucket::RateLimiter;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
struct ConversionUnits {
    #[serde(default)]
    liters: Option<f32>,
    #[serde(default)]
    gallons: Option<f32>,
    #[serde(default)]
    litres: Option<f32>,
    #[serde(default)]
    pints: Option<f32>,
}

pub fn new_rate_limiter() -> RateLimiter {
    RateLimiter::builder()
        .initial(5)
        .refill(1)
        .interval(Duration::from_secs(1))
        .max(5)
        .build()
}

#[post("/9/milk")]
async fn milk(
    limiter: web::Data<Arc<Mutex<RateLimiter>>>,
    req: HttpRequest,
    data: String,
) -> impl Responder {
    // log data
    println!("req data: {:?}", data);
    // get content header
    let ct_header = req.headers().get(CONTENT_TYPE);
    println!("ct_header: {:?}", ct_header);
    let bucket = limiter.lock().await;
    let flag = bucket.try_acquire(1);
    if matches!(
        ct_header.map(|ct| ct.to_str()),
        Some(Ok("application/json"))
    ) {
        let conversion_unit = serde_json::from_str::<ConversionUnits>(&data);
        println!("Payload {:?}", conversion_unit);
        match conversion_unit {
            Ok(unit) => {
                // process the request
                match (unit.gallons, unit.liters, unit.litres, unit.pints) {
                    (Some(gallons), None, None, None) => {
                        let liters = gallons * 3.785_412_5;
                        println!("litres: {liters}");
                        HttpResponse::Ok().json(json!({"liters": liters}))
                    }
                    (None, Some(liters), None, None) => {
                        // multiplication should expand the size of the float
                        let gallons = liters * 0.26417;
                        println!("gallons: {gallons}");
                        HttpResponse::Ok().json(json!({"gallons": gallons }))
                    }
                    (None, None, Some(litres), None) => {
                        // dealing with UK values
                        let pints = litres * 1.759754;
                        println!("pints: {}", pints);
                        HttpResponse::Ok().json(json!({"pints": pints}))
                    }
                    (None, None, None, Some(pints)) => {
                        // dealing with UK values
                        let litres = pints * 0.568261291;
                        println!("litres: {}", litres);
                        HttpResponse::Ok().json(json!({"litres": litres}))
                    }
                    _ => HttpResponse::BadRequest().finish(),
                }
            }
            Err(_) => HttpResponse::BadRequest().finish(),
        }
    } else if flag {
        HttpResponse::Ok().body("Milk withdrawn\n")
    } else {
        HttpResponse::Ok()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body("No milk available\n")
    }
}
