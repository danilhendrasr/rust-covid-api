
use actix_web::{get, HttpRequest, Result};

#[get("")]
pub async fn index(_req: HttpRequest) -> Result<String> {
  Ok(String::from("monthly"))
}

#[get("/{year}")]
pub async fn specific_year(_req: HttpRequest) -> Result<String> {
  Ok(String::from("monthly/{year}"))
}

#[get("/{year}/{month}")]
pub async fn specific_month(_req: HttpRequest) -> Result<String> {
  Ok(String::from("monthly/{year}/{month}"))
}
