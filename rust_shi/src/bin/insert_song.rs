#![allow(unused)]
use color_print::cprintln;
use std::{io::Write, str::FromStr};
use uuid::Uuid;

#[derive(Debug)]
struct Song {
    ant_id: Uuid,
    title: String,
    artist: String,
    release_year: i32,
}

impl Song {
    fn new() -> Self {
        Self {
            ant_id: Self::get_ant_id(),
            title: Self::get_title(),
            artist: "oke".to_string(),
            release_year: 2894
        }
    }

    fn get_ant_id() -> Uuid {
        println!("----------------------------------------");
        loop {
            print!("Enter the anth_id: ");
            std::io::stdout().flush().unwrap();
            let mut anth_id_str = String::new();
            std::io::stdin().read_line(&mut anth_id_str).unwrap();
            let anth_id_result = Uuid::from_str(anth_id_str.trim());
            match anth_id_result {
                Ok(anth_id) => {
                    return anth_id;
                }
                Err(err) => cprintln!("<red>Invalid UUID!</red> Please try again."),
            }
        }
    }

    fn get_title() -> String {
        println!("----------------------------------------");
        loop {
            print!("Enter the song title: ");
            std::io::stdout().flush().unwrap();
            let mut title_str = String::new();
            std::io::stdin().read_line(&mut title_str).unwrap();
            let title = title_str.trim().to_lowercase();
            if !title.is_empty() {
                return title;
            }
            cprintln!("<red>Title cannot be empty!</red>. Please try again");
        }
    }
}

fn main() {
    let song = Song::new();
    println!("{:?}", song);

    // let uuidku = "b2a6935e-1f8c-42e9-a528-e84dd0b6b89a";
    // let parsekan = Uuid::parse_str(&uuidku);
    // println!("{:?}", parsekan);
}
