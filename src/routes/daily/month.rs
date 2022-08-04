use super::types::{DailyEndpointError, DailyQueryParams};
use crate::utils::fetch_data_from_source_api;

use actix_web::{get, web, HttpResponse};
use chrono::NaiveDate;

/// Get all daily cases in a month.
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
        (status = 404, description = "There are no case yet for the given year or a given month", body = String),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
#[get("/{year}/{month}")]
pub async fn all_days_in_a_month(
    params: web::ReqData<DailyQueryParams>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, DailyEndpointError> {
    let (selected_year, selected_month) = path.into_inner();

    let params = params.into_inner();
    let mut daily_cases = fetch_data_from_source_api()
        .await
        .map_err(DailyEndpointError::UnexpectedError)?
        .to_daily()
        .get_all_daily_cases_in_a_month(selected_year, selected_month)
        .map_err(DailyEndpointError::NotFound)?
        .0;

    if let Some(since) = params.since {
        daily_cases = daily_cases
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

    Ok(HttpResponse::Ok().json(daily_cases))
}
