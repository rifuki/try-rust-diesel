use crate::models;
use crate::services::user;
use crate::utils::database::Pool;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_users(pool: web::Data<Pool>) -> impl Responder {
    match user::get_all_users(&pool).await {
        Ok(all_users) => HttpResponse::Ok().json(all_users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub async fn create_user(
    pool: web::Data<Pool>,
    user: web::Json<models::user::CustomUserPayload>,
) -> impl Responder {
    match user::create_user(&pool, &user.into_inner()).await {
        Ok(inserted_user) => HttpResponse::Created().json(inserted_user),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}
