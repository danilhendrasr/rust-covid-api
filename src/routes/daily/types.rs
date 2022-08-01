use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, derive_more::Display)]
pub enum DailyEndpointError {
    #[display(fmt = "{}", _0)]
    UnexpectedError(String),
    #[display(fmt = "{}", _0)]
    NotFound(String),
}

impl From<reqwest::Error> for DailyEndpointError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnexpectedError(err.to_string())
    }
}

impl ResponseError for DailyEndpointError {
    fn error_response(&self) -> HttpResponse {
        let mut http_response = match self {
            DailyEndpointError::UnexpectedError(_) => HttpResponse::InternalServerError(),
            DailyEndpointError::NotFound(_) => HttpResponse::NotFound(),
        };

        http_response.body(self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct DailyQueryParams {
    pub since: Option<YearMonthDay>,
    pub upto: Option<YearMonthDay>,
}

#[derive(Debug, Clone)]
pub struct YearMonthDay {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}
