use crate::schema::code_names::dsl::code_names as code_names_table;
use crate::{crud::code_name::create::CodeNameCli, models::code_name::CodeName};
use diesel::{result::Error, PgConnection, RunQueryDsl, SelectableHelper};

pub fn create_code_name(
    conn: &mut PgConnection,
    code_name: &CodeNameCli,
) -> Result<CodeName, Error> {
    let new_code_name = CodeName {
        id: code_name.id.to_string(),
    };

    let inserted_user = diesel::insert_into(code_names_table)
        .values(new_code_name)
        .returning(CodeName::as_returning())
        .get_result(conn)?;

    Ok(inserted_user)
}
