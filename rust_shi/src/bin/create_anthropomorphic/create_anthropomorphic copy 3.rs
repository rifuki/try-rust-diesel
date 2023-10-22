#![allow(unused)]
use color_print::cprintln;
use rust_shi::{
    models::anthropomorphism_model::{Gender, HairColor},
    services::anthropomorphism_service::create_anthropomorphic,
    utils::{
        converter::{str_to_gender, str_to_hair_color},
        database::establish_connection,
    },
};
use std::io::Write;

struct AnthropomorphicCharacter {
    name: String,
    age: i32,
    gender: Gender,
    hair_color: HairColor,
}

impl AnthropomorphicCharacter {
    fn new() -> Self {
        Self {
            name: Self::get_name(),
            age: Self::get_age(),
            gender: Self::get_gender(),
            hair_color: Self::get_hair_color(),
        }
    }

    fn get_name() -> String {
        println!("----------------------------------------");
        loop {
            print!("Enter anthropomorphic character name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_lowercase();
            if !name.is_empty() {
                return name;
            }
            cprintln!("<red>Name cannot be empty</red>. Please try again!");
        }
    }

    fn get_age() -> i32 {
        println!("----------------------------------------");
        loop {
            print!("Enter anthropomorphic character age: ");
            std::io::stdout().flush().unwrap();
            let mut age_str = String::new();
            std::io::stdin().read_line(&mut age_str).unwrap();
            if let Ok(age) = age_str.trim().parse::<i32>() {
                if age > 0 {
                    return age;
                }
            }
            cprintln!("<red>Invalid age!</red>. Age must be positive number. Please try again.");
        }
    }

    fn get_gender() -> Gender {
        println!("----------------------------------------");
        loop {
            print!(
                "Enter anthropomorphic character gender[{}|{}]: ",
                Gender::Male,
                Gender::Female
            );
            std::io::stdout().flush().unwrap();
            let mut gender_str = String::new();
            std::io::stdin().read_line(&mut gender_str).unwrap();
            let gender_result = str_to_gender(&gender_str);
            match gender_result {
                Ok(gender) => return gender,
                Err(err) => cprintln!("<red>Invalid gender!</red> {}", err),
            }
        }
    }

    fn get_hair_color() -> HairColor {
        println!("----------------------------------------");
        loop {
            print!(
                "Enter anthropomorphic character hair color[{}|{}|{}|{}|{}]: ",
                HairColor::Turquoise,
                HairColor::Blue,
                HairColor::Green,
                HairColor::Pink,
                HairColor::Yellow
            );
            std::io::stdout().flush().unwrap();
            let mut hair_color_str = String::new();
            std::io::stdin().read_line(&mut hair_color_str).unwrap();
            let hair_color_result = str_to_hair_color(&hair_color_str);
            match hair_color_result {
                Ok(hair_color) => return hair_color,
                Err(err) => cprintln!("<red>Invalid hair color!</red> {}", err),
            }
        }
    }
}

fn main() {
    let mut conn = establish_connection();

    let ant_chara = AnthropomorphicCharacter::new();

    let result = create_anthropomorphic(
        &mut conn,
        ant_chara.name.clone(),
        ant_chara.age,
        ant_chara.gender,
        ant_chara.hair_color.clone(),
    )
    .unwrap_or_else(|err| {
        cprintln!(
            "An error occured while saving character {}\n\r ({})",
            ant_chara.name,
            err
        );
        std::process::exit(1);
    });

    // print colorfull
    match ant_chara.hair_color {
        HairColor::Turquoise => cprintln!(
            "Saved a character <bold><cyan>{}</cyan></bold> with id <bold>{}</bold>",
            ant_chara.name,
            result.id
        ),
        HairColor::Blue => cprintln!(
            "Saved a character <bold><blue>{}</blue></bold> with id <bold>{}</bold>",
            ant_chara.name,
            result.id
        ),
        HairColor::Pink => cprintln!(
            "Saved a character <bold><bright-magenta>{}</bright-magenta></bold> with id <bold>{}</bold>",
            ant_chara.name,
            result.id
        ),
        HairColor::Green => cprintln!(
            "Saved a character <bold><green>{}</green></bold> with id <bold>{}</bold>",
            ant_chara.name,
            result.id
        ),
        HairColor::Yellow => cprintln!(
            "Saved a character <bold><yellow>{}</yellow></bold> with id <bold>{}</bold>",
            ant_chara.name,
            result.id
        ),
    }
}
