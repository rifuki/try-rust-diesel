use actix_web::web::{ServiceConfig, self};
use crate::controllers::user_controller::{get_all_users, create_user, get_single_user, delete_user};

pub fn scoped_user(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
        .route("", web::get().to(get_all_users))
        .route("", web::post().to(create_user))
        .service(
            web::resource("/{user_id}")
            .route(web::get().to(get_single_user))
            .route(web::delete().to(delete_user))
        )
    );
}