use std::num::ParseIntError;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    HttpMessage,
};
use actix_web_lab::middleware::Next;

use crate::types::QueryParams;

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

pub async fn filter_malformed_query_params(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let query_string = req.query_string();
    let query_params = serde_urlencoded::from_str::<QueryParams>(query_string)?;

    let mut monthly_query_params = MonthlyQueryParams {
        since: None,
        upto: None,
    };

    if let Some(since) = query_params.since {
        let splitted_params = since.split('-').collect::<Vec<_>>();

        if splitted_params.len() == 2 {
            let splitted_params = splitted_params
                .iter()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>();

            if let Ok(params) = splitted_params {
                let (year, month) = (params[0], params[1]);
                monthly_query_params.since = Some(YearMonth {
                    year,
                    month: month as u32,
                });
            }
        }
    }

    if let Some(upto) = query_params.upto {
        let splitted_params = upto.split('-').collect::<Vec<_>>();

        if splitted_params.len() == 2 {
            let splitted_params = splitted_params
                .iter()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>();

            if let Ok(params) = splitted_params {
                let (year, month) = (params[0], params[1]);
                monthly_query_params.upto = Some(YearMonth {
                    year,
                    month: month as u32,
                });
            }
        }
    }

    req.extensions_mut().insert(monthly_query_params);
    next.call(req).await
}
