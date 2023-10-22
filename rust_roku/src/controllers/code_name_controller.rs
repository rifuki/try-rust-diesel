use actix_web::{web, HttpResponse};

use crate::{
    models::code_name_model::CodeName,
    services::{
        code_name_service::{
            get_all_code_names as get_all_code_names_service,
            insert_code_name as insert_code_name_service,
        },
        code_name_service_error::CodeNameServiceError,
    },
    utils::database::Pool,
};

pub async fn get_all_code_names(pool: web::Data<Pool>) -> HttpResponse {
    match get_all_code_names_service(&pool).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => match err {
            CodeNameServiceError::ValidationError(msg) => {
                HttpResponse::Conflict().body(format!("{}", msg))
            }
            CodeNameServiceError::ConnectionError(msg) => {
                HttpResponse::InternalServerError().body(format!("{}", msg))
            }
            CodeNameServiceError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().body(format!("{}", msg))
            }
        },
    }
}

pub async fn insert_code_name(pool: web::Data<Pool>, payload: web::Json<CodeName>) -> HttpResponse {
    match insert_code_name_service(&pool, payload.into_inner()).await {
        Ok(inserted_code_name) => HttpResponse::Created().body(format!(
            "sucessfully inserted the code-name {}",
            inserted_code_name.id.to_uppercase()
        )),
        Err(err) => match err {
            CodeNameServiceError::ValidationError(msg) => {
                HttpResponse::Conflict().body(format!("{}", msg))
            }
            CodeNameServiceError::ConnectionError(msg) => {
                HttpResponse::InternalServerError().body(format!("{}", msg))
            }
            CodeNameServiceError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().body(format!("{}", msg))
            }
        },
    }
}
