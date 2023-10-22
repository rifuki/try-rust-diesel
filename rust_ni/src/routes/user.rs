use actix_web::{web, HttpResponse};
use crate::controllers::user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(user::get_users))
            .route("", web::post().to(user::create_user))
            .service(
                web::resource("/{user_id}")
                    .route(web::put().to(|| async { dummy_res("put") }))
                    .route(web::get().to(|| async { dummy_res("get single") }))
                    .route(web::delete().to(|| async { dummy_res("delete") })),
            ),
    );
}

fn dummy_res(kotoba: &str) -> HttpResponse {
    HttpResponse::Ok().body(format!("[{}] user response", kotoba.to_uppercase()))
}
