use super::types::{MonthlyEndpointError, MonthlyQueryParams};
use crate::utils::fetch_data_from_source_api;

use actix_web::{get, web, HttpResponse};
use chrono::NaiveDate;
use chrono_utilities::naive::DateTransitions;

/// Get all monthly cases in a year.
#[utoipa::path(
    context_path = "/monthly",
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
            description = "In ISO 8601 format but take the year and month only (YYYY-MM).",
            example = "2021-03"
        ),
        (
            "upto" = Option<String>,
            query,
            description = "In ISO 8601 format but take the year and month only (YYYY-MM).",
            example = "2022-07"
        )
    ),
    responses(
        (status = 200, description = "Success getting the data.", body = [MonthlyCase]),
        (status = 404, description = "There are no case yet for the given year", body = String),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
#[get("/{year}")]
pub async fn all_months_in_a_year(
    params: web::ReqData<MonthlyQueryParams>,
    path: web::Path<i32>,
) -> Result<HttpResponse, MonthlyEndpointError> {
    let selected_year = path.into_inner();
    let params = params.into_inner();
    let mut daily_cases = fetch_data_from_source_api()
        .await
        .map_err(MonthlyEndpointError::UnexpectedError)?
        .to_daily();

    if let Some(since) = params.since {
        daily_cases = daily_cases
            .0
            .into_iter()
            .filter(|daily| {
                let daily_date = NaiveDate::from_ymd(daily.year, daily.month, daily.day);
                let since_date = NaiveDate::from_ymd(since.year, since.month, 1);

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
                let upto_date = NaiveDate::from_ymd(upto.year, upto.month, 1);
                // Because upto_date is used to filter daily cases,
                // thus we need to create a NaiveDate object that correctly points
                // to the last day of the month in order to get all daily cases in a given month.
                let upto_date =
                    NaiveDate::from_ymd(upto.year, upto.month, upto_date.last_day_of_month());

                let num_of_days_till_upto = current_daily_date
                    .signed_duration_since(upto_date)
                    .num_days();

                num_of_days_till_upto <= 0
            })
            .collect();
    }

    Ok(HttpResponse::Ok().json(
        daily_cases
            .get_all_months_in_a_year(selected_year)
            .map_err(MonthlyEndpointError::NotFound)?
            .0,
    ))
}
