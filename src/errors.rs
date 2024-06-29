
use core::fmt;
use std::fmt::{Display, Formatter};
use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;

#[derive(Debug)]
pub enum BlogAppError{
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl From<SQLxError> for BlogAppError {
    fn from(err: SQLxError) -> Self {
        BlogAppError::DBError(err.to_string())
    }
}

impl Display for BlogAppError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl BlogAppError {
    fn err_response(&self) -> String {
        match self {
            BlogAppError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            BlogAppError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            BlogAppError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            BlogAppError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}


impl error::ResponseError for BlogAppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            BlogAppError::DBError(_msg) | BlogAppError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            BlogAppError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            BlogAppError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.err_response(),
        })
    }
}