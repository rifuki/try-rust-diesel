use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE URL must be set.");
        std::process::exit(1);
    });

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder().build(manager).unwrap()
}
