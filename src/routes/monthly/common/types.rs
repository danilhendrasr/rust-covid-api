use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, derive_more::Display)]
pub enum MonthlyEndpointError {
    #[display(fmt = "{}", _0)]
    UnexpectedError(String),
    #[display(fmt = "{}", _0)]
    NotFound(String),
}

impl From<reqwest::Error> for MonthlyEndpointError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnexpectedError(err.to_string())
    }
}

impl ResponseError for MonthlyEndpointError {
    fn error_response(&self) -> HttpResponse {
        let mut http_response = match self {
            MonthlyEndpointError::UnexpectedError(_) => HttpResponse::InternalServerError(),
            MonthlyEndpointError::NotFound(_) => HttpResponse::NotFound(),
        };

        http_response.body(self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct MonthlyQueryParams {
    pub since: Option<YearMonth>,
    pub upto: Option<YearMonth>,
}

#[derive(Debug, Clone)]
pub struct YearMonth {
    pub year: i32,
    pub month: u32,
}
