use super::types::{DailyEndpointError, DailyQueryParams};
use crate::utils::fetch_data_from_source_api;

use actix_web::{get, web, HttpResponse};
use chrono::NaiveDate;

/// Get all daily cases in a year.
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
            "since" = Option<String>,
            query,
            description = "In ISO 8601 format (YYYY-MM-DD).",
            example = "2021-03-03"
        ),
        (
            "upto" = Option<String>,
            query,
            description = "In ISO 8601 format (YYYY-MM-DD).",
            example = "2021-04-01"
        ),
    ),
    responses(
        (status = 200, description = "Success getting the data.", body = [DailyCase]),
        (status = 404, description = "There are no cases yet for the given year.", body = String),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
#[get("/{year}")]
pub async fn all_days_in_a_year(
    params: web::ReqData<DailyQueryParams>,
    path: web::Path<i32>,
) -> Result<HttpResponse, DailyEndpointError> {
    let selected_year = path.into_inner();

    let params = params.into_inner();
    let mut daily_cases = fetch_data_from_source_api()
        .await
        .map_err(DailyEndpointError::UnexpectedError)?
        .to_daily();

    if let Some(since) = params.since {
        daily_cases = daily_cases
            .0
            .into_iter()
            .filter(|daily| {
                let daily_date = NaiveDate::from_ymd(daily.year, daily.month, daily.day);
                let since_date = NaiveDate::from_ymd(since.year, since.month, since.day);

                let num_of_days_after_since =
                    daily_date.signed_duration_since(since_date).num_days();

                num_of_days_after_since >= 0
            })
            .collect();
    }

    if let Some(upto) = params.upto {
        daily_cases = daily_cases
            .0
            .into_iter()
            .filter(|daily| {
                let current_daily_date = NaiveDate::from_ymd(daily.year, daily.month, daily.day);
                let upto_date = NaiveDate::from_ymd(upto.year, upto.month, upto.day);

                let num_of_days_till_upto = current_daily_date
                    .signed_duration_since(upto_date)
                    .num_days();

                num_of_days_till_upto <= 0
            })
            .collect();
    }

    Ok(HttpResponse::Ok().json(
        &daily_cases
            .get_all_days_in_a_year(selected_year)
            .map_err(DailyEndpointError::NotFound)?
            .0,
    ))
}
