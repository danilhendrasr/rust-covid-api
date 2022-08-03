use super::types::{DailyEndpointError, DailyQueryParams};
use crate::{types::YearMonthPath, utils::fetch_data_from_source_api};

use actix_web::{get, http::header::ContentType, web, HttpResponse};
use chrono::NaiveDate;

#[get("/{year}/{month}")]
pub async fn specific_month(
    params: web::ReqData<DailyQueryParams>,
    path: web::Path<YearMonthPath>,
) -> Result<HttpResponse, DailyEndpointError> {
    let params = params.into_inner();
    let mut daily_cases = fetch_data_from_source_api()
        .await
        .map_err(DailyEndpointError::UnexpectedError)?
        .to_daily()
        .get_all_daily_cases_in_a_month(path.year, path.month)
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

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&daily_cases).unwrap()))
}
