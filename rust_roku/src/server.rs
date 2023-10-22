use crate::{routes::code_name_route::code_name, utils::database::establish_connection};
use actix_web::{dev::Server, middleware::NormalizePath, web, App, HttpServer};

pub fn start_server() -> Result<Server, std::io::Error> {
    let pool = establish_connection();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .configure(code_name)
    })
    .bind(("::", 80))?;

    let address = "http://localhost";
    println!("Server is beating up at {}", address);

    Ok(server.run())
}
