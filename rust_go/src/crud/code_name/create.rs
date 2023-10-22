use crate::{
    services::code_name::create::create_code_name as insert_code_name,
    utils::cli::{get_user_input, SEPARATOR_EQ},
};
use color_print::cprintln;
use diesel::PgConnection;

pub struct CodeNameCli {
    pub id: String,
}

impl CodeNameCli {
    fn new() -> Self {
        Self {
            id: Self::get_code_name(),
        }
    }

    fn get_code_name() -> String {
        println!("{}", SEPARATOR_EQ);
        loop {
            let code_name = get_user_input("Enter the Code-Name Vocaloid : ");
            if !code_name.is_empty() {
                return code_name;
            }
            cprintln!("<s><r>Code-Name cannot be empty!</r> <w>Please try again.</w></s>\n");
        }
    }
}

pub fn create_code_name(conn: &mut PgConnection) {
    let code_name = CodeNameCli::new();

    match insert_code_name(conn, &code_name) {
        Ok(result) => cprintln!(
            "✅ <g>Code-name <s>{}</s> successfully inserted.</g>✅",
            result.id.to_uppercase()
        ),
        Err(err) => cprintln!(
            "❗️ <s><r>An error occurred while inserting code-name {}</r></s> ❗️\n\r<R><s>{}</s></R>",
            code_name.id,
            err
        ),
    }
}
