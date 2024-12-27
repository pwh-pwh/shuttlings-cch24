use std::ffi::c_long;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use actix_web::cookie::Cookie;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    data: String, // Arbitrary JSON data
    exp: usize,
}

#[post("/16/wrap")]
async fn wrap(data: String) -> impl Responder {
    println!("{}", data);
    // 获取当前时间戳
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600;
    let claims = Claims { data: data.clone(),exp: exp };
    let token = encode(&Header::default(),&claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
//     set cookie
    println!("{}", token);
    HttpResponse::Ok().cookie(Cookie::build("gift",token).finish()).finish()
}


#[get("/16/unwrap")]
async fn unwrap(req: HttpRequest) -> impl Responder {
    println!("unwrap");
    if let Some(cookie) = req.cookie("gift") {
        let token = cookie.value();
        println!("token: {}", token);
        let result = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()),&Validation::default()).unwrap();
        println!("result: {}", result.claims.data);
        HttpResponse::Ok().body(result.claims.data)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use crate::day16::Claims;

    #[test]
    fn test_jwt() {
        /*let body = "hello";
        let claims = Claims { body: body.to_string(),exp: 9999999999 };
        let token = encode(&Header::default(),&claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
        println!("token: {}", token);
        let result = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()),&Validation::default()).unwrap();
        println!("result: {}", result.claims.body);*/
    }
}