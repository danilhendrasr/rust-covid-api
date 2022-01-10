use actix_web::{get, web, HttpResponse};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct YearPath {
  year: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Update {
  harian: Vec<crate::api_types::Harian>,
}

#[derive(Serialize, Debug)]
struct ResponseStructure {
  year: String,
  positive: i32,
  recovered: i32,
  deaths: i32,
  active: i32,
}

#[derive(Deserialize)]
pub struct IndexQueryParams {
  since: Option<String>,
  upto: Option<String>,
}

#[get("")]
pub async fn index(params: web::Query<IndexQueryParams>) -> HttpResponse {
  let parsed_since_param = params
    .since
    .clone()
    .unwrap_or(String::from("0"))
    .parse::<u16>()
    .unwrap_or(0);

  let parsed_upto_param = params
    .upto
    .clone()
    .unwrap_or(String::from("-1"))
    .parse::<u16>()
    .unwrap_or(0);

  let valid_years = HashSet::from(crate::constants::YEARS_LIST);
  if (!params.since.is_none() && !valid_years.contains(&parsed_since_param))
    || (!params.upto.is_none() && !valid_years.contains(&parsed_upto_param))
  {
    return HttpResponse::BadRequest()
      .status(reqwest::StatusCode::BAD_REQUEST)
      .body("Invalid query parameter(s)");
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct APIResponse {
    update: Update,
  }

  type HandlerResponse = crate::api_types::HandlerResponse<Vec<ResponseStructure>>;
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
            year: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .format("%Y")
              .to_string(),
            positive: daily_item.jumlah_positif.value as i32,
            recovered: daily_item.jumlah_sembuh.value as i32,
            deaths: daily_item.jumlah_meninggal.value as i32,
            active: daily_item.jumlah_dirawat.value as i32,
          })
          .collect();

        let new_harian: Vec<ResponseStructure> = new_harian
          .into_iter()
          .filter(|daily| {
            if !params.since.is_none() && !params.upto.is_none() {
              return daily.year.parse::<u16>().unwrap() >= parsed_since_param
                && daily.year.parse::<u16>().unwrap() <= parsed_upto_param;
            }

            if !params.since.is_none() {
              return daily.year.parse::<u16>().unwrap() >= parsed_since_param;
            }

            if !params.upto.is_none() {
              return daily.year.parse::<u16>().unwrap() <= parsed_upto_param;
            }

            true
          })
          .collect();

        let mut years_list: Vec<u16> = new_harian
          .iter()
          .map(|daily_item| daily_item.year.parse::<u16>().unwrap())
          .collect::<HashSet<u16>>()
          .into_iter()
          .collect();

        years_list.sort();
        let mut to_return: Vec<ResponseStructure> = Vec::new();
        years_list.iter().for_each(|year| {
          let folded = new_harian
            .iter()
            .filter(|daily| daily.year == *year.to_string())
            .fold(
              ResponseStructure {
                year: year.to_string(),
                positive: 0,
                recovered: 0,
                deaths: 0,
                active: 0,
              },
              |mut acc, next| {
                acc.positive += next.positive;
                acc.recovered += next.recovered;
                acc.deaths += next.deaths;
                acc.active += next.active;
                acc
              },
            );

          to_return.push(folded);
        });

        HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
          serde_json::to_string(&HandlerResponse {
            ok: true,
            data: to_return,
            message: String::from("success"),
          })
          .unwrap(),
        )
      }
    },
  }
}

#[get("/{year}")]
pub async fn specific_year(path: web::Path<YearPath>) -> HttpResponse {
  type HandlerResponse = crate::api_types::HandlerResponse<ResponseStructure>;
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
            year: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
              .unwrap()
              .format("%Y")
              .to_string(),
            positive: daily_item.jumlah_positif.value as i32,
            recovered: daily_item.jumlah_sembuh.value as i32,
            deaths: daily_item.jumlah_meninggal.value as i32,
            active: daily_item.jumlah_dirawat.value as i32,
          })
          .collect();

        let to_return = new_harian.iter().fold(
          ResponseStructure {
            year: selected_year.to_string(),
            positive: 0,
            recovered: 0,
            deaths: 0,
            active: 0,
          },
          |mut accumulator, next| {
            accumulator.positive += next.positive;
            accumulator.recovered += next.recovered;
            accumulator.deaths += next.deaths;
            accumulator.active += next.active;
            accumulator
          },
        );

        HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
          serde_json::to_string(&HandlerResponse {
            ok: true,
            data: to_return,
            message: String::from("success"),
          })
          .unwrap(),
        )
      }
    },
  }
}
