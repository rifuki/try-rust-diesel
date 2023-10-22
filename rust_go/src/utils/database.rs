use color_print::cprintln;
use diesel::{pg::PgConnection, Connection};
// use diesel::Connection;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        cprintln!("<red><bold>DATABASE_URL must be set.</bold><red>");
        std::process::exit(1);
    });

    PgConnection::establish(&database_url).unwrap_or_else(|err| {
        cprintln!("<red><bold>{}</bold></red>", err);
        std::process::exit(1);
    })
}
