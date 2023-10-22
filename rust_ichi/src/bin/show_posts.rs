use std::io::Write;

use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rust_ichi::establish_connection;
use rust_ichi::models;
use rust_ichi::schema;

fn main() {
    use schema::posts::dsl::posts;
    let connection = &mut establish_connection();

    let results = posts
        .select(models::Post::as_select())
        .get_results(connection)
        .unwrap_or_else(|err| {
            eprintln!("Error loading post. ({})", err);
            std::process::exit(1);
        });

    for result in &results {
        println!("id: {}", result.id);
        println!("title: {}", result.title);
        println!("body: {}", result.body);
        println!("published: {}", result.published);
        println!("created_at: {}", result.created_at);
        println!("updated_at: {}", result.updated_at);
    }

    let mut file = std::fs::File::create("result.txt").unwrap();
    if results.len() <= 0 {
        file.write_all("🔥 KOSONG 🔥".as_bytes()).unwrap();
    } else {
        for result in results {
            let line = format!(
            "ID: {} | title: {} | body: {} | published: {} | created_at: {} | updated_at: {}\n\r\n\r\n\r",
            result.id,
            result.title,
            result.body,
            result.published,
            result.created_at,
            result.updated_at
        );
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}
