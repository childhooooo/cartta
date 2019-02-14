use rocket::http::Status;
use bcrypt::*;
use std::fmt;
use validator::{ValidationError, ValidationErrors};

#[derive(Debug)]
pub enum AccountError {
    IncorrectPassword,
    InvalidUser,
    BcryptError(BcryptError),
    NotFound,
    DatabaseError(diesel::result::Error),
    ValidationError(ValidationError),
    ValidationErrors(ValidationErrors)
}

impl From<BcryptError> for AccountError {
    fn from(error: BcryptError) -> AccountError {
        AccountError::BcryptError(error)
    }
}

impl From<ValidationError> for AccountError {
    fn from(error: ValidationError) -> AccountError {
        AccountError::ValidationError(error)
    }
}

impl From<ValidationErrors> for AccountError {
    fn from(error: ValidationErrors) -> AccountError {
        AccountError::ValidationErrors(error)
    }
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountError::IncorrectPassword => write!(f, "{}", "AccountError: IncorrectPassword"),
            AccountError::InvalidUser => write!(f, "{}", "AccountError: InvalidUser"),
            AccountError::BcryptError(ref error) => write!(f, "{}", error),
            AccountError::NotFound => write!(f, "{}", "AccountError: NotFound"),
            AccountError::DatabaseError(ref error) => write!(f, "{}", error),
            AccountError::ValidationError(ref error) => write!(f, "{}", error),
            AccountError::ValidationErrors(ref error) => write!(f, "{}", error)
        }
    }
}

impl From<AccountError> for Status {
    fn from(item: AccountError) -> Self {
        match item {
            AccountError::IncorrectPassword => Status::Unauthorized,
            AccountError::InvalidUser => Status::BadRequest,
            AccountError::BcryptError(ref error) => Status::InternalServerError,
            AccountError::NotFound => Status::NotFound,
            AccountError::DatabaseError(ref error) => Status::InternalServerError,
            AccountError::ValidationError(ref error) => Status::BadRequest,
            AccountError::ValidationErrors(ref error) => Status::BadRequest
        }
    }
}