use actix_web::{get, web, HttpResponse};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct ResponseStructure {
  date: String,
  positive: i32,
  recovered: i32,
  deaths: i32,
  active: i32,
}

type HandlerResponse = crate::api_types::HandlerResponse<Vec<ResponseStructure>>;

#[derive(Serialize, Deserialize, Debug)]
struct Update {
  harian: Vec<crate::api_types::Harian>,
}

#[derive(Deserialize)]
pub struct QueryParams {
  _since: Option<String>,
  _upto: Option<String>,
}

#[get("")]
pub async fn index(_info: web::Query<QueryParams>) -> HttpResponse {
  #[derive(Serialize, Deserialize, Debug)]
  struct APIResponse {
    update: Update,
  }

  let resp = reqwest::get(crate::constants::COVID_API_ENDPOINT).await;
  match resp {
    Err(_) => HttpResponse::InternalServerError()
      .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
      .body("Could not get the data, please retry in a few minutes"),

    Ok(raw_response) => match raw_response.json::<APIResponse>().await {
      Err(_) => HttpResponse::InternalServerError()
        .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        .body("There's something wrong with us, hang tight"),

      Ok(json_response) => {
        let new_harian: Vec<ResponseStructure> = json_response
          .update
          .harian
          .into_iter()
          .map(|daily_item| ResponseStructure {
            date: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .format("%Y-%m-%d")
              .to_string(),
            positive: daily_item.jumlah_positif.value as i32,
            recovered: daily_item.jumlah_sembuh.value as i32,
            deaths: daily_item.jumlah_meninggal.value as i32,
            active: daily_item.jumlah_dirawat.value as i32,
          })
          .collect();

        HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
          serde_json::to_string(&HandlerResponse {
            ok: true,
            data: new_harian,
            message: String::from("success"),
          })
          .unwrap(),
        )
      }
    },
  }
}

#[derive(Deserialize)]
pub struct YearPath {
  year: u16,
}

#[get("/{year}")]
pub async fn specific_year(path: web::Path<YearPath>) -> HttpResponse {
  #[derive(Serialize, Deserialize, Debug)]
  struct APIResponse {
    update: Update,
  }

  let selected_year = path.year;

  let resp = reqwest::get(crate::constants::COVID_API_ENDPOINT).await;
  match resp {
    Err(_) => HttpResponse::InternalServerError()
      .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
      .body("Could not get the data, please retry in a few minutes"),

    Ok(raw_response) => match raw_response.json::<APIResponse>().await {
      Err(_) => HttpResponse::InternalServerError()
        .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        .body("There's something wrong with us, hang tight"),

      Ok(json_response) => {
        let new_harian: Vec<ResponseStructure> = json_response
          .update
          .harian
          .into_iter()
          .filter(|daily_item| {
            DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .year()
              == selected_year as i32
          })
          .map(|daily_item| ResponseStructure {
            date: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .format("%Y-%m-%d")
              .to_string(),
            positive: daily_item.jumlah_positif.value as i32,
            recovered: daily_item.jumlah_sembuh.value as i32,
            deaths: daily_item.jumlah_meninggal.value as i32,
            active: daily_item.jumlah_dirawat.value as i32,
          })
          .collect();

        HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
          serde_json::to_string(&HandlerResponse {
            ok: true,
            data: new_harian,
            message: String::from("success"),
          })
          .unwrap(),
        )
      }
    },
  }
}

#[derive(Deserialize)]
pub struct YearMonthPath {
  year: u16,
  month: u8,
}

#[get("/{year}/{month}")]
pub async fn specific_month(path: web::Path<YearMonthPath>) -> HttpResponse {
  #[derive(Serialize, Deserialize, Debug)]
  struct APIResponse {
    update: Update,
  }

  let selected_year = path.year;
  let selected_month = path.month;

  let resp = reqwest::get(crate::constants::COVID_API_ENDPOINT).await;
  match resp {
    Err(_) => HttpResponse::InternalServerError()
      .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
      .body("Could not get the data, please retry in a few minutes"),

    Ok(raw_response) => match raw_response.json::<APIResponse>().await {
      Err(_) => HttpResponse::InternalServerError()
        .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        .body("There's something wrong with us, hang tight"),

      Ok(json_response) => {
        let new_harian: Vec<ResponseStructure> = json_response
          .update
          .harian
          .into_iter()
          .filter(|daily_item| {
            DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .year()
              == selected_year as i32
              && DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                .unwrap()
                .month()
                == selected_month as u32
          })
          .map(|daily_item| ResponseStructure {
            date: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .format("%Y-%m-%d")
              .to_string(),
            positive: daily_item.jumlah_positif.value as i32,
            recovered: daily_item.jumlah_sembuh.value as i32,
            deaths: daily_item.jumlah_meninggal.value as i32,
            active: daily_item.jumlah_dirawat.value as i32,
          })
          .collect();

        HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
          serde_json::to_string(&HandlerResponse {
            ok: true,
            data: new_harian,
            message: String::from("success"),
          })
          .unwrap(),
        )
      }
    },
  }
}

#[derive(Deserialize)]
pub struct YearMonthDatePath {
  year: u16,
  month: u8,
  date: u8,
}

#[get("/{year}/{month}/{date}")]
pub async fn specific_date(path: web::Path<YearMonthDatePath>) -> HttpResponse {
  #[derive(Serialize, Deserialize, Debug)]
  struct APIResponse {
    update: Update,
  }

  let selected_year = path.year;
  let selected_month = path.month;
  let selected_date = path.date;

  let resp = reqwest::get(crate::constants::COVID_API_ENDPOINT).await;
  match resp {
    Err(_) => HttpResponse::InternalServerError()
      .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
      .body("Could not get the data, please retry in a few minutes"),

    Ok(raw_response) => match raw_response.json::<APIResponse>().await {
      Err(_) => HttpResponse::InternalServerError()
        .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        .body("There's something wrong with us, hang tight"),

      Ok(json_response) => {
        let new_harian: Vec<ResponseStructure> = json_response
          .update
          .harian
          .into_iter()
          .filter(|daily_item| {
            DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .year()
              == selected_year as i32
              && DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                .unwrap()
                .month()
                == selected_month as u32
              && DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                .unwrap()
                .day()
                == selected_date as u32
          })
          .map(|daily_item| ResponseStructure {
            date: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .format("%Y-%m-%d")
              .to_string(),
            positive: daily_item.jumlah_positif.value as i32,
            recovered: daily_item.jumlah_sembuh.value as i32,
            deaths: daily_item.jumlah_meninggal.value as i32,
            active: daily_item.jumlah_dirawat.value as i32,
          })
          .collect();

        HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
          serde_json::to_string(&HandlerResponse {
            ok: true,
            data: new_harian,
            message: String::from("success"),
          })
          .unwrap(),
        )
      }
    },
  }
}
