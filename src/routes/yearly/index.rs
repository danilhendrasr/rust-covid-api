use super::{common::types::QueryParams, errors::YearlyEndpointError};
use crate::utils::fetch_data_from_source_api;
use actix_web::{get, web, HttpResponse};

#[get("")]
pub async fn index(params: web::Query<QueryParams>) -> Result<HttpResponse, YearlyEndpointError> {
    let mut daily_cases = fetch_data_from_source_api()
        .await
        .map_err(YearlyEndpointError::UnexpectedError)?
        .to_daily();

    if let Some(since) = params.since {
        daily_cases = daily_cases
            .0
            .into_iter()
            .filter(|daily| daily.year >= since)
            .collect();
    }

    if let Some(upto) = params.upto {
        daily_cases = daily_cases
            .0
            .into_iter()
            .filter(|daily| daily.year <= upto)
            .collect();
    }

    Ok(HttpResponse::Ok().json(daily_cases.to_yearly().0))
}
