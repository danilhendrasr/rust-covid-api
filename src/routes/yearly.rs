
use actix_web::{get, HttpRequest, Result};

#[get("")]
pub async fn index(_req: HttpRequest) -> Result<String> {
  Ok(String::from("yearly"))
}

#[get("/{year}")]
pub async fn specific_year(_req: HttpRequest) -> Result<String> {
  Ok(String::from("yearly/{year}"))
}
