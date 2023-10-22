use rust_ichi::{create_post, establish_connection};
use std::io::{Read, Write};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    print!("What would you like your title to be? ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    println!("Ok! let's write {} (Press {} when finished)", title, EOF);
    std::io::stdin().read_to_string(&mut body).unwrap();
    println!("Wokeeeeeeeeeeeeeee");
    let body = body.trim();

    let result = create_post(connection, title, body);
    println!("Saved draft {} with id: {}", result.title, result.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
