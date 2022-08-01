use crate::types;

pub static COVID_API_ENDPOINT: &str = "https://data.covid19.go.id/public/api/update.json";

pub async fn fetch_data_from_source_api() -> Result<types::source_api::Response, String> {
    let resp = reqwest::get(COVID_API_ENDPOINT)
        .await
        .map_err(|_| "Failed fetching data from source API.")?;

    let json = resp.json().await.map_err(|_| "Failed processing data.")?;

    Ok(json)
}
