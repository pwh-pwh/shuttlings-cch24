use actix_web::cookie::Cookie;
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use jsonwebtoken::{decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ffi::c_long;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::errors::{Error, ErrorKind};

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
    let claims = Claims {
        data: data.clone(),
        exp: exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();
    //     set cookie
    println!("{}", token);
    HttpResponse::Ok()
        .cookie(Cookie::build("gift", token).finish())
        .finish()
}

#[get("/16/unwrap")]
async fn unwrap(req: HttpRequest) -> impl Responder {
    println!("unwrap");
    if let Some(cookie) = req.cookie("gift") {
        let token = cookie.value();
        println!("token: {}", token);
        let result = decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .unwrap();
        println!("result: {}", result.claims.data);
        HttpResponse::Ok().body(result.claims.data)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[post("/16/decode")]
async fn decode_endpoint(jwt: String) -> impl Responder {
    println!("start");
    let pem = include_str!("static/santa_public_key.pem");
    let Ok(key) = DecodingKey::from_rsa_pem(pem.as_bytes()) else {
        return HttpResponse::InternalServerError().finish();
    };
    let Ok(_)  = decode_header(&jwt) else {
        return HttpResponse::BadRequest().finish();
    };
    println!("header ok");
    let mut validation = Validation::default();
    validation.validate_exp = true;
    validation.algorithms = vec![Algorithm::RS256, Algorithm::RS512];
    validation.set_required_spec_claims(&[""]);
    match decode::<serde_json::Value>(&jwt, &key, &validation) {
        Ok(result) => HttpResponse::Ok().json(result.claims),
        Err(e) => match e.kind() {
            ErrorKind::InvalidSignature => {
                HttpResponse::Unauthorized().finish()
            },
            _ => {
                println!("error: {}", e);
                HttpResponse::BadRequest().finish() },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::Claims;
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

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
