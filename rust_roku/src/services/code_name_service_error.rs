use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use std::convert::From;

struct ErrJsonMsg {
    error: String,
    message: String,
}

#[derive(Debug)]
pub enum CodeNameServiceError {
    ConnectionError(R2d2Error),
    DatabaseError(DieselError),
    ValidationError(String)
}

impl CodeNameServiceError {
    fn new(&self) -> ErrJsonMsg {
        let err_msg = match self {
            CodeNameServiceError::ConnectionError(err) => err.to_string(),
            CodeNameServiceError::DatabaseError(err) => err.to_string(),
            CodeNameServiceError::ValidationError(err) => err.to_string()
        };

        ErrJsonMsg {
            error: "Error".to_string(),
            message: err_msg.to_string(),
        }
    }
}

impl From<R2d2Error> for CodeNameServiceError {
    fn from(value: R2d2Error) -> Self {
        Self::ConnectionError(value)
    }
}

impl From<DieselError> for CodeNameServiceError {
    fn from(value: DieselError) -> Self {
        Self::DatabaseError(value)
    }
}