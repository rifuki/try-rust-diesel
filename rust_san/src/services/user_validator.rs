use crate::{
    models::user_model::User as UserModel,
    schema::users::dsl::{username as username_column, users as users_table},
    services::user_service_error::UserServiceError,
    utils::database::Pool,
};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

pub async fn check_username_unique(pool: &Pool, username: &str) -> Result<bool, UserServiceError> {
    let mut conn = pool.get()?;

    let result = users_table
        .filter(username_column.eq(username))
        .first::<UserModel>(&mut conn)
        .is_ok();

    Ok(!result)
}
