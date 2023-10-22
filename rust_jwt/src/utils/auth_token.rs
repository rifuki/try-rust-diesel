use crate::routes::token::Claims;
use actix_web::{
    dev::Payload, error, http::header::AUTHORIZATION, web, Error as ActixWebError, FromRequest,
    HttpRequest,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::future::{ready, Ready};
pub struct AuthToken {
    pub id: usize,
}

impl FromRequest for AuthToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get(AUTHORIZATION);
        let auth_token = auth_header.map_or("", |hv| hv.to_str().unwrap_or(""));

        if auth_token.is_empty() {
            return ready(Err(error::ErrorUnauthorized("Token is empty!")));
        }

        let secret = req.app_data::<web::Data<String>>().unwrap();
        let decoded_token = decode::<Claims>(
            auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decoded_token {
            Ok(token) => ready(Ok(AuthToken {
                id: token.claims.id,
            })),
            Err(err) => ready(Err(error::ErrorUnauthorized(format!(
                "Unauthorized! : {}",
                err
            )))),
        }
    }
}
