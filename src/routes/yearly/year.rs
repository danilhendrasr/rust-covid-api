use super::errors::YearlyEndpointError;
use crate::utils::fetch_data_from_source_api;

use actix_web::{get, web, HttpResponse};

/// Get a specific year's case.
#[utoipa::path(
    context_path = "/yearly",
    tag = "Data",
    params(("year", description = "Get the given year's case.", example = 2021)),
    responses(
        (status = 200, description = "Success getting the given year's case.", body = YearlyCase),
        (status = 404, description = "There are no cases yet for the given year", body = String),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
#[get("/{year}")]
pub async fn specific_year(
    year: web::Path<i32>,
) -> actix_web::Result<HttpResponse, YearlyEndpointError> {
    let selected_year = year.into_inner();

    let daily = fetch_data_from_source_api()
        .await
        .map_err(YearlyEndpointError::UnexpectedError)?
        .to_daily();

    Ok(HttpResponse::Ok().json(
        daily
            .to_specific_yearly(selected_year)
            .map_err(YearlyEndpointError::ResourceNotFound)?,
    ))
}
