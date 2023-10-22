use super::reads::read_all_code_names;
use crate::{
    services::code_name::delete::delete_code_name as remove_code_name, utils::cli::get_user_input,
};
use color_print::cprintln;
use diesel::PgConnection;

fn get_code_name() -> String {
    loop {
        let code_name = get_user_input("Enter the code-name vocaloid want to delete: ");
        if !code_name.is_empty() {
            return code_name;
        }
        cprintln!(
            "<s><r>Expected a code-name to match against!</r> <w>Please try again.</w></s>\n"
        );
    }
}

pub fn delete_code_name(conn: &mut PgConnection) {
    read_all_code_names(conn);
    let code_name = get_code_name();

    match remove_code_name(conn, &code_name) {
        Ok(deleted_code_name) => {
            cprintln!(
                "✅ <g>Code-name <s>{}</s> successfully deleted.</g>✅",
                deleted_code_name.id.to_uppercase()
            )
        }
        Err(err) => cprintln!(
            "❗️ <s><r>An error occurred while deleting code-name {}</r></s> ❗️\n\r<R><s>{}</s></R>",
            code_name.to_uppercase(),
            err
        ),
    }
}
