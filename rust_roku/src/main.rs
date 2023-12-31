use rust_roku::server::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server()?.await
}
