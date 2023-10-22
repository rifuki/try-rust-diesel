use crate::{
    models::user_model::{CustomUserPayload, RoleEnum, User},
    schema::users::dsl::{id as id_column, users as users_table},
    services::{user_service_error::UserServiceError, user_validator::check_username_unique},
    utils::database::Pool,
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

pub async fn get_all_users(pool: &Pool) -> Result<Vec<User>, UserServiceError> {
    let mut conn = pool.get()?;

    let result = users_table.load(&mut conn)?;

    Ok(result)
}

pub async fn create_user(
    pool: &Pool,
    payload: CustomUserPayload,
) -> Result<User, UserServiceError> {
    if payload.username.is_none() || payload.password.is_none() {
        return Err(UserServiceError::ValidationError(
            "username and password can't be empty".to_string(),
        ));
    }

    let is_username_unique = check_username_unique(pool, &payload.username.clone().unwrap()).await?;
    if !is_username_unique {
        return Err(UserServiceError::ValidationError("Username is already taken".to_owned()));
    }

    let mut conn = pool.get()?;

    let new_user = User {
        id: Uuid::new_v4(),
        username: payload.username.unwrap(),
        password: payload.password.unwrap(),
        role: payload.role.unwrap_or_else(|| RoleEnum::User),
    };

    let result = diesel::insert_into(users_table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)?;

    Ok(result)
}

pub async fn get_single_user(pool: &Pool, user_id: Uuid) -> Result<Option<User>, UserServiceError> {
    let mut conn = pool.get()?;

    let result = users_table
        .filter(id_column.eq(user_id))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?;

    Ok(result)
}

pub async fn delete_user(pool: &Pool, user_id: Uuid) -> Result<User, UserServiceError> {
    let mut conn = pool.get()?;

    let result = diesel::delete(users_table.find(user_id)).get_result(&mut conn)?;

    Ok(result)
}
