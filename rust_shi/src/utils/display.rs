use crate::models::anthropomorphism_model::{Gender, HairColor};
use std::fmt;

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_name = match self {
            Gender::Male => "male",
            Gender::Female => "female",
        };
        write!(f, "{}", variant_name)
    }
}

impl fmt::Display for HairColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_name = match self {
            HairColor::Turquoise => "turquoise",
            HairColor::Pink => "pink",
            HairColor::Yellow => "yellow",
            HairColor::Blue => "blue",
            HairColor::Green => "green",
        };
        write!(f, "{}", variant_name)
    }
}
