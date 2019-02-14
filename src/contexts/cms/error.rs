use rocket::http::Status;
use std::fmt;
use validator::{ValidationError, ValidationErrors};

#[derive(Debug)]
pub enum CMSError {
    NotFound,
    DatabaseError(diesel::result::Error),
    ValidationError(ValidationError),
    ValidationErrors(ValidationErrors)
}

impl From<ValidationError> for CMSError {
    fn from(error: ValidationError) -> CMSError {
        CMSError::ValidationError(error)
    }
}

impl From<ValidationErrors> for CMSError {
    fn from(error: ValidationErrors) -> CMSError {
        CMSError::ValidationErrors(error)
    }
}

impl fmt::Display for CMSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CMSError::NotFound => write!(f, "{}", "CMSError: NotFound"),
            CMSError::DatabaseError(ref error) => write!(f, "{}", error),
            CMSError::ValidationError(ref error) => write!(f, "{}", error),
            CMSError::ValidationErrors(ref error) => write!(f, "{}", error)
        }
    }
}

impl From<CMSError> for Status {
    fn from(item: CMSError) -> Self {
        match item {
            CMSError::NotFound => Status::NotFound,
            CMSError::DatabaseError(ref error) => Status::InternalServerError,
            CMSError::ValidationError(ref error) => Status::BadRequest,
            CMSError::ValidationErrors(ref error) => Status::BadRequest
        }
    }
}