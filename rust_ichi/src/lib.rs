pub mod models;
pub mod schema;

use diesel::{pg::PgConnection, Connection, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("DATABASE_URL must be set. ({})", err);
        std::process::exit(1);
    });

    PgConnection::establish(&database_url).unwrap_or_else(|err| {
        eprintln!("Error connecting to {} ({})", database_url, err);
        std::process::exit(1);
    })
}

pub fn create_post(connection: &mut PgConnection, title: &str, body: &str) -> models::Post {
    let new_post = models::NewPost {
        id: Some(Uuid::new_v4()),
        created_at: chrono::Local::now().naive_utc(),
        updated_at: chrono::Local::now().naive_utc(),
        title,
        body,
    };

    diesel::insert_into(schema::posts::table)
        .values(new_post)
        .returning(models::Post::as_returning())
        .get_result(connection)
        .unwrap_or_else(|err| {
            eprintln!("Error creating new post ({})", err);
            std::process::exit(1);
        })
}
