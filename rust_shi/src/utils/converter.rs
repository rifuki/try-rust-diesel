use crate::models::anthropomorphism_model::{Gender, HairColor};

pub fn str_to_gender(input: &str) -> Result<Gender, String> {
    match input.trim().to_lowercase().as_str() {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err("Gender must be between male or female.".to_string()),
    }
}

pub fn str_to_hair_color(input: &str) -> Result<HairColor, String> {
    match input.trim().to_lowercase().as_str() {
        "turquoise" => Ok(HairColor::Turquoise),
        "pink" => Ok(HairColor::Pink),
        "yellow" => Ok(HairColor::Yellow),
        "green" => Ok(HairColor::Green),
        "blue" => Ok(HairColor::Blue),
        _ => Err(format!(
            "Hair Color must be between [ {} | {} | {} | {} | {} ].",
            HairColor::Turquoise,
            HairColor::Pink,
            HairColor::Blue,
            HairColor::Green,
            HairColor::Yellow
        )),
    }
}
