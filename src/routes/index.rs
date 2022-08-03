use crate::utils::fetch_data_from_source_api;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct IndexEndpointResponse {
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

pub async fn index_handler() -> Result<HttpResponse, SlashEndpointError> {
    let y = fetch_data_from_source_api()
        .await
        .map_err(SlashEndpointError::UnexpectedError)?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(
        serde_json::to_string(&IndexEndpointResponse {
            total_positive: y.update.total.jumlah_positif,
            total_recovered: y.update.total.jumlah_sembuh,
            total_deaths: y.update.total.jumlah_meninggal,
            total_active: y.update.total.jumlah_dirawat,
            new_positive: y.update.penambahan.jumlah_positif,
            new_recovered: y.update.penambahan.jumlah_sembuh,
            new_deaths: y.update.penambahan.jumlah_meninggal,
            new_active: y.update.penambahan.jumlah_dirawat,
        })
        .unwrap(),
    ))
}
