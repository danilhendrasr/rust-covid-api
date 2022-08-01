use super::errors::YearlyEndpointError;
use crate::{types::YearPath, utils::fetch_data_from_source_api};

use actix_web::{get, web, HttpResponse};

#[get("/{year}")]
pub async fn specific_year(
    path: web::Path<YearPath>,
) -> actix_web::Result<HttpResponse, YearlyEndpointError> {
    let selected_year = path.year;

    let daily = fetch_data_from_source_api()
        .await
        .map_err(YearlyEndpointError::UnexpectedError)?
        .to_daily();

    Ok(HttpResponse::Ok().body(
        serde_json::to_string(
            &daily
                .to_specific_yearly(selected_year)
                .map_err(YearlyEndpointError::ResourceNotFound)?,
        )
        .unwrap(),
    ))
}
