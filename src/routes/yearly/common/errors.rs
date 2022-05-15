use actix_web::{HttpResponse, ResponseError};
use std::fmt::Debug;

#[derive(Debug, derive_more::Display)]
pub enum YearlyEndpointError {
    #[display(fmt = "{}", _0)]
    BadRequest(String),
    #[display(fmt = "{}", _0)]
    UnexpectedError(String),
}

impl From<reqwest::Error> for YearlyEndpointError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnexpectedError(err.to_string())
    }
}

impl ResponseError for YearlyEndpointError {
    fn error_response(&self) -> HttpResponse {
        let mut http_response = match self {
            YearlyEndpointError::BadRequest(_) => HttpResponse::BadRequest(),
            _ => HttpResponse::InternalServerError(),
        };

        http_response.body(self.to_string())
    }
}