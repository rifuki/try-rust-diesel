use crate::{models::code_name::CodeName, schema::code_names::dsl::code_names as code_names_table};
use diesel::{result::Error, PgConnection, RunQueryDsl};

pub fn read_all_code_names(conn: &mut PgConnection) -> Result<Vec<CodeName>, Error> {
    let get_code_names = code_names_table.load::<CodeName>(conn)?;

    Ok(get_code_names)

    // let query = diesel::sql_query("SELECT * FROM code_names");
    // let get_code_names = query.get_results::<CodeName>(conn)?;
}
