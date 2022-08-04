use super::types::MonthlyEndpointError;
use crate::utils::fetch_data_from_source_api;

use actix_web::{get, web, HttpResponse};

/// Get a specific month's case.
#[utoipa::path(
    context_path = "/monthly",
    tag = "Data",
    params(
        ("year", description = "Selected year.", example = 2021),
        ("month", description = "Selected month.", example = 7)
    ),
    responses(
        (status = 200, description = "Success getting the data.", body = MonthlyCase),
        (status = 404, description = "There are no case yet for the given year or a given month", body = String),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
#[get("/{year}/{month}")]
pub async fn specific_month(
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MonthlyEndpointError> {
    let (selected_year, selected_month) = path.into_inner();

    let daily_cases = fetch_data_from_source_api()
        .await
        .map_err(MonthlyEndpointError::UnexpectedError)?
        .to_daily();

    Ok(HttpResponse::Ok().json(
        daily_cases
            .get_specific_month(selected_year, selected_month)
            .map_err(MonthlyEndpointError::NotFound)?,
    ))
}
