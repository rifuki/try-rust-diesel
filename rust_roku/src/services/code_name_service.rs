use crate::{
    models::code_name_model::CodeName, schema::code_names::dsl::code_names as code_names_table,
    services::code_name_service_error::CodeNameServiceError, utils::database::Pool,
};
use diesel::{RunQueryDsl, SelectableHelper};

use super::code_name_service_validator::check_code_name_id_unique;

pub async fn get_all_code_names(pool: &Pool) -> Result<Vec<CodeName>, CodeNameServiceError> {
    let mut conn = pool.get()?;

    let code_names = code_names_table.load::<CodeName>(&mut conn)?;

    Ok(code_names)
}

pub async fn insert_code_name(
    pool: &Pool,
    payload: CodeName,
) -> Result<CodeName, CodeNameServiceError> {
    let mut conn = pool.get()?;

    let is_code_name_unique = check_code_name_id_unique(pool, payload.id.clone()).await;
    if !is_code_name_unique {
        return Err(CodeNameServiceError::ValidationError(format!(
            "The provided code-name {} already exists in database.",
            payload.id.to_uppercase()
        )));
    }

    let new_code_name = CodeName { id: payload.id };

    let inserted_code_name = diesel::insert_into(code_names_table)
        .values(new_code_name)
        .returning(CodeName::as_returning())
        .get_result(&mut conn)?;

    Ok(inserted_code_name)
}
