pub mod types {
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
}

pub mod middleware {
    use super::types::{DailyQueryParams, YearMonthDay};
    use crate::types::QueryParams;

    use actix_web::{
        body::MessageBody,
        dev::{ServiceRequest, ServiceResponse},
        HttpMessage,
    };
    use actix_web_lab::middleware::Next;
    use std::num::ParseIntError;

    pub async fn filter_malformed_query_params(
        req: ServiceRequest,
        next: Next<impl MessageBody>,
    ) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
        let query_string = req.query_string();
        let query_params = serde_urlencoded::from_str::<QueryParams>(query_string)?;

        let mut daily_query_params = DailyQueryParams {
            since: None,
            upto: None,
        };

        if let Some(since) = query_params.since {
            let splitted_params = since.split('-').collect::<Vec<_>>();

            if splitted_params.len() == 3 {
                let splitted_params = splitted_params
                    .iter()
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, ParseIntError>>();

                if let Ok(params) = splitted_params {
                    let (year, month, day) = (params[0], params[1], params[2]);
                    daily_query_params.since = Some(YearMonthDay {
                        year,
                        month: month as u32,
                        day: day as u32,
                    });
                }
            }
        }

        if let Some(upto) = query_params.upto {
            let splitted_params = upto.split('-').collect::<Vec<_>>();

            if splitted_params.len() == 3 {
                let splitted_params = splitted_params
                    .iter()
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, ParseIntError>>();

                if let Ok(params) = splitted_params {
                    let (year, month, day) = (params[0], params[1], params[2]);
                    daily_query_params.upto = Some(YearMonthDay {
                        year,
                        month: month as u32,
                        day: day as u32,
                    });
                }
            }
        }

        req.extensions_mut().insert(daily_query_params);
        next.call(req).await
    }
}
