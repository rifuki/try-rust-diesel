use std::io::{stdin, stdout, Write};

pub fn get_user_input(message: &str) -> String {
    print!("{}", message);
    stdout().flush().unwrap();
    let mut user_input_str = String::new();
    stdin().read_line(&mut user_input_str).unwrap();
    user_input_str.trim().to_lowercase()
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
}

pub const SEPARATOR_DASH: &str = "--------------------------------";
pub const SEPARATOR_EQ: &str =   "================================";
