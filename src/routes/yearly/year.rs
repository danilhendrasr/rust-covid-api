use super::errors::YearlyEndpointError;
use crate::{
    types::{self, YearPath},
    utils::fetch_data_from_source_api,
};

use actix_web::{get, web, HttpResponse};

type HandlerResponse = types::HandlerResponseTemplate<types::YearlyItem>;

#[get("/{year}")]
pub async fn specific_year(
    path: web::Path<YearPath>,
) -> actix_web::Result<HttpResponse, YearlyEndpointError> {
    let selected_year = path.year;

    let daily = fetch_data_from_source_api()
        .await
        .map_err(YearlyEndpointError::UnexpectedError)?
        .to_daily();

    let specific_year = daily
        .to_specific_yearly(selected_year)
        .map_err(YearlyEndpointError::ResourceNotFound)?;

    Ok(HttpResponse::Ok().body(
        serde_json::to_string(&HandlerResponse {
            ok: true,
            data: specific_year,
            message: "success".to_string(),
        })
        .unwrap(),
    ))
}
