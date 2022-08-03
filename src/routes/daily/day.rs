use super::types::DailyEndpointError;
use crate::{types::YearMonthDayPath, utils::fetch_data_from_source_api};

use actix_web::{get, http::header::ContentType, web, HttpResponse};

#[get("/{year}/{month}/{day}")]
pub async fn specific_day(
    path: web::Path<YearMonthDayPath>,
) -> Result<HttpResponse, DailyEndpointError> {
    let daily_case = fetch_data_from_source_api()
        .await
        .map_err(DailyEndpointError::UnexpectedError)?
        .to_daily()
        .get_specific_day(path.year, path.month, path.day)
        .map_err(DailyEndpointError::NotFound)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&daily_case).unwrap()))
}
