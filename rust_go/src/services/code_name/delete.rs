use diesel::{result::Error, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{models::code_name::CodeName, schema::code_names::dsl::code_names as code_names_table};

pub fn delete_code_name(conn: &mut PgConnection, code_name_id: &str) -> Result<CodeName, Error> {
    let deleted_code_name = diesel::delete(code_names_table.find(code_name_id))
        .returning(CodeName::as_returning())
        .get_result(conn)?;

    Ok(deleted_code_name)
}
