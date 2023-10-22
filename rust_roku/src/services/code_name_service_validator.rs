use diesel::{QueryDsl, RunQueryDsl};

use crate::{
    models::code_name_model::CodeName, schema::code_names::dsl::code_names as code_names_table,
    utils::database::Pool,
};

pub async fn check_code_name_id_unique(pool: &Pool, id: String) -> bool {
    let mut conn = pool.get().unwrap();

    let is_code_id_unique = code_names_table
        .find(id)
        .first::<CodeName>(&mut conn)
        .is_ok();

    return !is_code_id_unique;
}
