use rust_shi::{
    models::anthropomorphism_model::{Gender, HairColor},
    services::anthropomorphism_service::create_anthropomorphic,
    utils::{
        converter::{str_to_gender, str_to_hair_color},
        database::establish_connection,
    },
};
use std::io::Write;

fn main() {
    let mut conn = establish_connection();

    let mut name = String::new();
    let new_name: String;
    let mut age = String::new();
    let new_age: i32;
    let mut gender = String::new();
    let new_gender: Gender;
    let mut hair_color = String::new();
    let new_hair_color: HairColor;

    println!("------------------------------------");
    loop {
        print!("Enter anthropomorphic character name: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut name).unwrap();
        let name_shadow = name.trim().to_lowercase();
        if !name_shadow.is_empty() {
            new_name = name_shadow.clone();
            break;
        }
        eprintln!("😠 Name cannot be empty. Please try again. 😠");
    }

    println!("------------------------------------");
    loop {
        print!("Enter anthropomorphic character age: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut age).unwrap();
        let age_shadow = age.trim().parse::<i32>();
        match age_shadow {
            Ok(a) if a > 0 => {
                new_age = a;
                break;
            }
            Ok(_) => {
                String::clear(&mut age);
                eprintln!("😡 Invalid age! Age must be positive number. Please try again. 😡")
            }
            Err(err) => {
                String::clear(&mut age);
                eprintln!("😡 Invalid age! Please enter a valid number. 😡 <{}>", err)
            }
        }
    }

    println!("------------------------------------");
    loop {
        print!(
            "Enter anthropomorphic character gender [{:?} | {:?}]: ",
            Gender::Male,
            Gender::Female
        );
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut gender).unwrap();
        let gender_shadow = str_to_gender(&gender);
        match gender_shadow {
            Ok(g) => {
                new_gender = g;
                break;
            }
            Err(err) => {
                String::clear(&mut gender);
                eprintln!("{}", err)
            }
        }
    }

    println!("------------------------------------");
    loop {
        print!(
            "Enter anthropomorphic character hair color [{:?} | {:?} | {:?} | {:?} | {:?}]: ",
            HairColor::Turquoise,
            HairColor::Pink,
            HairColor::Blue,
            HairColor::Green,
            HairColor::Yellow
        );
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut hair_color).unwrap();
        let hair_color_shadow = str_to_hair_color(&hair_color);
        match hair_color_shadow {
            Ok(hc) => {
                new_hair_color = hc;
                break;
            }
            Err(err) => {
                String::clear(&mut hair_color);
                eprintln!("{}", err)
            }
        }
    }

    let result =
        create_anthropomorphic(&mut conn, new_name.clone(), new_age, new_gender, new_hair_color)
            .unwrap_or_else(|err| {
                eprintln!("😭 Error saving character {} 😭 <{}>", new_name, err);
                std::process::exit(1);
            });

    println!(
        "😁 Saved character {} with id {} 😁",
        result.name, result.id
    );
}
