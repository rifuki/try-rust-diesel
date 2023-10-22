// services/user_service.rs

use crate::models;
use crate::schema::users::dsl::users;
use crate::utils::database::Pool;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use std::convert::From;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

pub async fn get_all_users(pool: &Pool) -> Result<Vec<models::user::User>, UserServiceError> {
    let mut conn = pool.get().map_err(UserServiceError::ConnectionError)?;
    let all_users = users
        .load::<models::user::User>(&mut conn)
        .map_err(UserServiceError::DatabaseError)?;

    Ok(all_users)
}
// services/user_service.rs

pub async fn create_user(
    pool: &Pool,
    user: &models::user::CustomUserPayload,
) -> Result<models::user::User, UserServiceError> {
    let mut conn = pool.get().map_err(UserServiceError::ConnectionError)?;

    let new_user = models::user::User {
        id: Uuid::new_v4(),
        username: user.username.clone(),
        password: user.password.clone(),
        role: user.role.clone(),
    };

    let inserted_user = diesel::insert_into(users)
        .values(new_user)
        .returning(models::user::User::as_returning())
        .get_result(&mut conn)?;

    Ok(inserted_user)
}

#[derive(Debug)]
pub enum UserServiceError {
    DatabaseError(DieselError),
    ConnectionError(R2d2Error),
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserServiceError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            UserServiceError::ConnectionError(err) => write!(f, "Connection Error: {}", err),
        }
    }
}

impl Error for UserServiceError {}

impl From<DieselError> for UserServiceError {
    fn from(error: DieselError) -> Self {
        UserServiceError::DatabaseError(error)
    }
}

impl From<R2d2Error> for UserServiceError {
    fn from(error: R2d2Error) -> Self {
        UserServiceError::ConnectionError(error)
    }
}
