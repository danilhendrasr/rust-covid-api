use crate::{api_types, constants::*, domains::QueryParams, utils::get_json_data_from_source_api};
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
    harian: Vec<api_types::Harian>,
}

#[derive(Serialize, Debug)]
struct YearObject {
    year: String,
    positive: i32,
    recovered: i32,
    deaths: i32,
    active: i32,
}

struct Response(Vec<YearObject>);

impl Response {
    fn distinct_years(&self) -> Vec<u32> {
        let mut distinct_years = self
            .0
            .iter()
            .map(|daily_item| daily_item.year.parse::<u32>().unwrap())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        distinct_years.sort();
        distinct_years
    }

    pub fn _filter<T: Fn() -> bool>(self, _closure: T) -> Self {
        // TODO: Implement a filter function for easier filtering
        todo!()
    }

    pub fn fold_by_year(self) -> Self {
        let years_list = self.distinct_years();

        let mut to_return: Vec<YearObject> = Vec::new();
        years_list.iter().for_each(|year| {
            let folded = self
                .0
                .iter()
                .filter(|daily| daily.year == *year.to_string())
                .fold(
                    YearObject {
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

        Response(to_return)
    }
}

impl From<SourceAPIResponse> for Response {
    fn from(api_response: SourceAPIResponse) -> Self {
        let result = api_response
            .update
            .harian
            .iter()
            .map(|daily_item| YearObject {
                year: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                    .unwrap()
                    .format("%Y")
                    .to_string(),
                positive: daily_item.jumlah_positif.value as i32,
                recovered: daily_item.jumlah_sembuh.value as i32,
                deaths: daily_item.jumlah_meninggal.value as i32,
                active: daily_item.jumlah_dirawat.value as i32,
            })
            .collect::<Vec<YearObject>>();

        Response(result)
    }
}

impl FromIterator<YearObject> for Response {
    fn from_iter<I: IntoIterator<Item = YearObject>>(iter: I) -> Self {
        let mut holder: Vec<YearObject> = Vec::new();

        for i in iter {
            holder.push(i);
        }

        Self(holder)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SourceAPIResponse {
    update: Update,
}

#[get("")]
pub async fn index(params: web::Query<QueryParams>) -> actix_web::Result<HttpResponse> {
    let QueryParams { since, upto } = params.0;

    type HandlerResponse = api_types::HandlerResponse<Vec<YearObject>>;

    let source_api_resp =
        match get_json_data_from_source_api::<SourceAPIResponse>(COVID_API_ENDPOINT).await {
            Ok(value) => value,
            Err(message) => return Err(actix_web::error::ErrorInternalServerError(message)),
        };

    let daily_cases = Response::from(source_api_resp)
        .0
        .into_iter()
        .filter(|daily| {
            if let Some(value) = &since {
                return daily.year.parse::<u32>().unwrap() >= value.content()[0];
            }

            return true;
        })
        .filter(|daily| {
            if let Some(value) = &upto {
                return daily.year.parse::<u32>().unwrap() <= value.content()[0];
            }

            return true;
        })
        .collect::<Response>();

    let to_return = daily_cases.fold_by_year().0;

    Ok(HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
        serde_json::to_string(&HandlerResponse {
            ok: true,
            data: to_return,
            message: "success".to_string(),
        })
        .unwrap(),
    ))
}

#[get("/{year}")]
pub async fn specific_year(path: web::Path<YearPath>) -> actix_web::Result<HttpResponse> {
    type HandlerResponse = api_types::HandlerResponse<YearObject>;

    let selected_year = path.year;

    let raw_json_resp =
        match get_json_data_from_source_api::<SourceAPIResponse>(COVID_API_ENDPOINT).await {
            Ok(value) => value,
            Err(message) => return Err(actix_web::error::ErrorInternalServerError(message)),
        };

    let aggregated = Response::from(raw_json_resp)
        .0
        .into_iter()
        .filter(|daily_item| &daily_item.year.parse().unwrap() == selected_year as i32)
        .collect::<Response>()
        .fold_by_year();

    let to_return = match aggregated.0.into_iter().nth(0) {
        Some(value) => value,
        None => return Err(actix_web::error::ErrorInternalServerError(
            "Oops, we've made a mistake. Check back in a few minutes. Sorry for the inconvenience!",
        )),
    };

    Ok(HttpResponse::Ok().status(reqwest::StatusCode::OK).body(
        serde_json::to_string(&HandlerResponse {
            ok: true,
            data: to_return,
            message: "success".to_string(),
        })
        .unwrap(),
    ))
}
