use super::types::MonthlyEndpointError;
use crate::utils::fetch_data_from_source_api;

use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Path {
    year: i32,
    month: i32,
}

#[get("/{year}/{month}")]
pub async fn specific_month(path: web::Path<Path>) -> Result<HttpResponse, MonthlyEndpointError> {
    let daily_cases = fetch_data_from_source_api()
        .await
        .map_err(MonthlyEndpointError::UnexpectedError)?
        .to_daily();

    Ok(HttpResponse::Ok().body(
        serde_json::to_string(
            &daily_cases
                .get_specific_month(path.year, path.month)
                .map_err(MonthlyEndpointError::NotFound)?,
        )
        .unwrap(),
    ))
}
