use actix_web::{middleware::NormalizePath, web, App, HttpServer};
use rust_jwt::routes::token::scope_token;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::trim())
            .app_data(web::Data::new(String::from("secret_token")))
            .configure(scope_token)
    })
    .bind(("::", 80))?
    .run()
    .await
}
