use super::types::DailyEndpointError;
use crate::utils::fetch_data_from_source_api;

use actix_web::{get, web, HttpResponse};

/// Get a specific day's case.
#[utoipa::path(
    context_path = "/daily",
    tag = "Data",
    params(
        (
            "year",
            description = "Selected year.",
            example = 2021
        ),
        (
            "month",
            description = "Selected month.",
            example = 2
        ),
        (
            "day",
            description = "Selected day.",
            example = 26
        ),
    ),
    responses(
        (status = 200, description = "Success getting the data.", body = DailyCase),
        (status = 404, description = "There are no case yet for the given year, the given month, or the given day.", body = String),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
#[get("/{year}/{month}/{day}")]
pub async fn specific_day(
    path: web::Path<(i32, i32, i32)>,
) -> Result<HttpResponse, DailyEndpointError> {
    let (selected_year, selected_month, selected_day) = path.into_inner();

    let daily_case = fetch_data_from_source_api()
        .await
        .map_err(DailyEndpointError::UnexpectedError)?
        .to_daily()
        .get_specific_day(selected_year, selected_month, selected_day)
        .map_err(DailyEndpointError::NotFound)?;

    Ok(HttpResponse::Ok().json(daily_case))
}
