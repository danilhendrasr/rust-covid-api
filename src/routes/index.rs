use crate::utils::fetch_data_from_source_api;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Serialize, Debug, Deserialize, Component)]
#[component(example = json!({
    "total_positive": 6216621,
    "total_recovered": 6010545,
    "total_deaths": 157028,
    "total_active": 49048,
    "new_positive": 5827,
    "new_recovered": 4564,
    "new_deaths": 24,
    "new_active": 1239
}))]
pub struct CasesSummary {
    pub total_positive: u32,
    pub total_recovered: u32,
    pub total_deaths: u32,
    pub total_active: u32,
    pub new_positive: i64,
    pub new_recovered: i64,
    pub new_deaths: i64,
    pub new_active: i64,
}

#[derive(Debug, derive_more::Display)]
pub enum SlashEndpointError {
    #[display(fmt = "{}", _0)]
    UnexpectedError(String),
}

impl From<reqwest::Error> for SlashEndpointError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnexpectedError(err.to_string())
    }
}

impl ResponseError for SlashEndpointError {
    fn error_response(&self) -> HttpResponse {
        let mut http_response = HttpResponse::InternalServerError();
        http_response.body(self.to_string())
    }
}

/// Get summary of all daily cases.
#[utoipa::path(
    get,
    path = "/",
    tag = "Data",
    responses(
        (status = 200, description = "Success processing daily cases summary.", body = CasesSummary),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
pub async fn daily_cases_summary() -> Result<HttpResponse, SlashEndpointError> {
    let resp = fetch_data_from_source_api()
        .await
        .map_err(SlashEndpointError::UnexpectedError)?;

    Ok(HttpResponse::Ok().json(CasesSummary {
        total_positive: resp.update.total.jumlah_positif,
        total_recovered: resp.update.total.jumlah_sembuh,
        total_deaths: resp.update.total.jumlah_meninggal,
        total_active: resp.update.total.jumlah_dirawat,
        new_positive: resp.update.penambahan.jumlah_positif,
        new_recovered: resp.update.penambahan.jumlah_sembuh,
        new_deaths: resp.update.penambahan.jumlah_meninggal,
        new_active: resp.update.penambahan.jumlah_dirawat,
    }))
}
