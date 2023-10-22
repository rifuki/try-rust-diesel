use actix_web::web;

use crate::controllers::code_name_controller::{get_all_code_names, insert_code_name};

pub fn code_name(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/code_names")
            .route("", web::get().to(get_all_code_names))
            .route("", web::post().to(insert_code_name)),
    );
}
