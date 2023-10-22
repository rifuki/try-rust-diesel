use crate::{
    models::user_model::CustomUserPayload,
    services::{user_service, user_service_error::UserServiceError},
    utils::database::Pool,
};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

pub async fn get_all_users(pool: web::Data<Pool>) -> impl Responder {
    match user_service::get_all_users(&pool).await {
        Ok(all_users) => HttpResponse::Ok().json(all_users),
        Err(error) => {
            HttpResponse::InternalServerError().json(json!({ "msg": format!("{:?}", error) }))
        }
    }
}

pub async fn create_user(
    pool: web::Data<Pool>,
    payload: web::Json<CustomUserPayload>,
) -> impl Responder {
    match user_service::create_user(&pool, payload.into_inner()).await {
        Ok(inserted_user) => HttpResponse::Created().json(inserted_user),
        Err(error) => match error {
            UserServiceError::ValidationError(msg) => {
                HttpResponse::Forbidden().json(json!({ "msg": msg }))
            }
            _ => HttpResponse::InternalServerError().json(json!({ "msg": format!("{:?}", error) })),
        },
    }
}

pub async fn get_single_user(pool: web::Data<Pool>, user_id: web::Path<String>) -> impl Responder {
    match Uuid::parse_str(&user_id) {
        Ok(user_id) => match user_service::get_single_user(&pool, user_id).await {
            Ok(get_user) => {
                if let Some(user) = get_user {
                    HttpResponse::Ok().json(user)
                } else {
                    HttpResponse::NotFound().json(json!({"msg": "user not found"}))
                }
            }
            Err(error) => {
                HttpResponse::InternalServerError().json(json!({ "error": format!("{:?}", error) }))
            }
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_user(pool: web::Data<Pool>, user_id: web::Path<String>) -> impl Responder {
    match Uuid::parse_str(&user_id) {
        Ok(user_id) => match user_service::delete_user(&pool, user_id).await {
            Ok(delete_user) => HttpResponse::Ok().json(json!({
                "msg": format!("user {} successfully deleted", delete_user.username)
            })),
            Err(error) => {
                HttpResponse::InternalServerError().json(json!({ "error": format!("{:?}", error) }))
            }
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
