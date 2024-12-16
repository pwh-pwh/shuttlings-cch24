use std::sync::Arc;
use std::time::Duration;
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use leaky_bucket::RateLimiter;
use tokio::sync::Mutex;

pub fn new_rate_limiter() -> RateLimiter {
    RateLimiter::builder()
        .initial(5)
        .refill(1)
        .interval(Duration::from_secs(1))
        .max(5)
        .build()
}

#[post("/9/milk")]
async fn milk(limiter: web::Data<Arc<Mutex<RateLimiter>>>) -> impl Responder {
    let bucket = limiter.lock().await;
    if bucket.try_acquire(1) {
        HttpResponse::Ok().body("Milk withdrawn\n")
    } else {
     HttpResponse::Ok().status(StatusCode::TOO_MANY_REQUESTS).body("No milk available\n")   
    }
}