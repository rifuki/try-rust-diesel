use crate::utils::auth_token::AuthToken;
use actix_web::{
    error, http::header::AUTHORIZATION, web, Error as ActixWebError, HttpRequest, HttpResponse,
    Responder,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> impl Responder {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now().naive_utc() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();
    HttpResponse::Ok().json(EncodeResponse {
        message: String::from("success"),
        token,
    })
}

#[derive(Deserialize, Serialize)]
struct DecodeBody {
    token: String,
}

#[derive(Serialize)]
struct DecodeResponse {
    message: String,
    id: usize,
}

async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> impl Responder {
    let decoded: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(data) => {
            return HttpResponse::Ok().json(DecodeResponse {
                message: "authorize".to_string(),
                id: data.claims.id,
            })
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(DecodeResponse {
                message: e.to_string(),
                id: 0,
            })
        }
    };
}

#[derive(Serialize)]
struct Message<'a> {
    msg: &'a str,
}

async fn protected(auth_token: AuthToken) -> impl Responder {
    println!("{}", auth_token.id);
    HttpResponse::Ok().finish()
}

pub fn scope_token(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/token")
            .route("/protected", web::get().to(protected))
            .route("/{id}", web::get().to(encode_token))
            .route("", web::post().to(decode_token)),
    );
}
