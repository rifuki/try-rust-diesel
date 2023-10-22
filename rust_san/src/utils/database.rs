use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).unwrap_or_else(|err| {
        eprintln!("Error Building Pool [{}]", err);
        std::process::exit(1);
    })
}
