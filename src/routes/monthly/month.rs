use super::types::MonthlyEndpointError;
use crate::{types::YearMonthPath, utils::fetch_data_from_source_api};

use actix_web::{get, http::header::ContentType, web, HttpResponse};

#[get("/{year}/{month}")]
pub async fn specific_month(
    path: web::Path<YearMonthPath>,
) -> Result<HttpResponse, MonthlyEndpointError> {
    let daily_cases = fetch_data_from_source_api()
        .await
        .map_err(MonthlyEndpointError::UnexpectedError)?
        .to_daily();

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(
        serde_json::to_string(
            &daily_cases
                .get_specific_month(path.year, path.month)
                .map_err(MonthlyEndpointError::NotFound)?,
        )
        .unwrap(),
    ))
}
