use diesel::{pg::PgConnection, Connection};

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("DATABASE_URL must be set. [{}]", err);
        std::process::exit(1);
    });

    PgConnection::establish(&database_url).unwrap_or_else(|err| {
        eprintln!("Failed connecting to {} [{}]", database_url, err);
        std::process::exit(1)
    })
}
