use crate::{
    services::code_name::reads::read_all_code_names as get_all_code_names, utils::cli::SEPARATOR_EQ,
};
use color_print::cprintln;
use diesel::PgConnection;

pub fn read_all_code_names(conn: &mut PgConnection) {
    match get_all_code_names(conn) {
        Ok(code_names) => {
            cprintln!("<k><s>{}</s></k>", SEPARATOR_EQ);
            cprintln!(
                "{:<3} <w><s>Displaying {} Code-Names</s></w>",
                "",
                code_names.len()
            );
            cprintln!("<k><s>{}</s></k>", SEPARATOR_EQ);
            for code_name in code_names {
                cprintln!("Code-Name: {}", code_name.id.to_uppercase());
            }
            cprintln!("<k><s>{}</s></k>", SEPARATOR_EQ);
        }
        Err(err) => {
            cprintln!(
            "❗️ <s><r>An error occurred while fetching code-names</r></s> ❗️\n\r<R><s>{}</s></R>", err
            );
        }
    }
}
