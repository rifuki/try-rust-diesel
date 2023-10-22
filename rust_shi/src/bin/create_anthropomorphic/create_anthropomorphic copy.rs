use rust_shi::{
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
    let mut age = String::new();
    let mut gender = String::new();
    let mut hair_color = String::new();

    println!("------------------------------------");
    print!("Enter anthropomorphic character name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_lowercase();
    eprintln!("😠 Name cannot be empty. Please try again. 😠");

    println!("------------------------------------");
    print!("Enter anthropomorphic character age: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut age).unwrap();
    let age = age.trim().parse::<i32>().unwrap_or_else(|err| {
        eprintln!("😡 Invalid age! Please enter a valid number. 😡 <{}>", err);
        std::process::exit(1);
    });

    println!("------------------------------------");
    print!("Enter anthropomorphic character gender: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut gender).unwrap();
    let gender = str_to_gender(&gender).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    println!("------------------------------------");
    print!("Enter anthropomorphic character hair color: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut hair_color).unwrap();
    let hair_color = str_to_hair_color(&hair_color).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let result = create_anthropomorphic(&mut conn, name.clone(), age, gender, hair_color)
        .unwrap_or_else(|err| {
            eprintln!("😭 Error saving character {} 😭 <{}>", name, err);
            std::process::exit(1);
        });

    println!(
        "😁 Saved character {} with id {} 😁",
        result.name, result.id
    );
}
