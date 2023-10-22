use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use rust_san::{routes::user_router::scoped_user, utils::database::establish_connection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    calculate(12, 23);
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("DATABASE_URL must be set. [{}]", err);
        std::process::exit(1);
    });

    let pool = establish_connection(&database_url);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .configure(scoped_user)
    })
    .bind(("::", 80))?
    .run()
    .await
}

fn calculate(num1: i32, num2: i32) -> () {
    println!("{}", num1 + num2);
}
