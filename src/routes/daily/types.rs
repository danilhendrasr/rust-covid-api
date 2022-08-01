use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, derive_more::Display)]
pub enum DailyEndpointError {
    #[display(fmt = "{}", _0)]
    UnexpectedError(String),
}

impl From<reqwest::Error> for DailyEndpointError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnexpectedError(err.to_string())
    }
}

impl ResponseError for DailyEndpointError {
    fn error_response(&self) -> HttpResponse {
        let mut http_response = HttpResponse::InternalServerError();
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
