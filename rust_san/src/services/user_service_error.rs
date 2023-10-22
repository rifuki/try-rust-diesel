use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use std::convert::From;

#[derive(Debug)]
pub enum UserServiceError {
    ConnectionError(R2d2Error),
    DatabaseError(DieselError),
    ValidationError(String)
}

impl From<R2d2Error> for UserServiceError {
    fn from(value: R2d2Error) -> Self {
        UserServiceError::ConnectionError(value)
    }
}

impl From<DieselError> for UserServiceError {
    fn from(value: DieselError) -> Self {
        UserServiceError::DatabaseError(value)
    }
}

// impl std::fmt::Display for UserServiceError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             UserServiceError::ConnectionError(_) => write!(f, "Database connection error"),
//             UserServiceError::DatabaseError(_) => write!(f, "Database error"),
//             UserServiceError::ValidationError(msg) => write!(f, "{}", msg),
//         }
//     }
// }
