pub mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use std::fmt::Debug;

    #[derive(Debug, derive_more::Display)]
    pub enum YearlyEndpointError {
        #[display(fmt = "{}", _0)]
        BadRequest(String),
        #[display(fmt = "{}", _0)]
        UnexpectedError(String),
        #[display(fmt = "{}", _0)]
        ResourceNotFound(String),
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
                YearlyEndpointError::ResourceNotFound(_) => HttpResponse::NotFound(),
                _ => HttpResponse::InternalServerError(),
            };

            http_response.body(self.to_string())
        }
    }
}

pub mod types {
    use utoipa::IntoParams;

    #[derive(serde::Deserialize, Debug, IntoParams)]
    pub struct QueryParams {
        /// The bottom boundary of the yearly cases.
        #[param(example = 2020)]
        pub since: Option<i32>,
        /// The upper boundary of the yearly cases.
        #[param(example = 2021)]
        pub upto: Option<i32>,
    }
}
