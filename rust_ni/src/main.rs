use actix_web::web;
use actix_web::{middleware::NormalizePath, App, HttpServer};
use rust_ni::routes::user;
use rust_ni::utils::database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    println!("{:?}", rust_ni::models::user::UserRole::User);

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("DATABASE_URL must be set. [{}]", err);
        std::process::exit(1);
    });

    let pool = database::establish_connection(&database_url);

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .configure(user::configure)
    })
    .bind(("::", 80))?
    .run()
    .await
}
