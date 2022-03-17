use actix_web::{get, web, HttpResponse};
use chrono::prelude::*;
use chrono_utilities::naive::DateTransitions;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Debug)]
struct ResponseStructure {
    month: String,
    positive: i32,
    recovered: i32,
    deaths: i32,
    active: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Update {
    harian: Vec<crate::api_types::Harian>,
}

#[derive(Deserialize)]
pub struct YearPath {
    year: u16,
}

#[derive(Deserialize)]
pub struct YearMonthPath {
    year: u16,
    month: u8,
}

#[derive(Deserialize)]
pub struct QueryParams {
    since: Option<String>,
    upto: Option<String>,
}

#[derive(Debug)]
struct ParsedQueryParam {
    year: u16,
    month: u8,
}

impl std::clone::Clone for ResponseStructure {
    fn clone(&self) -> ResponseStructure {
        ResponseStructure {
            month: self.month.clone(),
            active: self.active,
            recovered: self.recovered,
            deaths: self.deaths,
            positive: self.positive,
        }
    }
}

#[get("")]
pub async fn index(params: web::Query<QueryParams>) -> HttpResponse {
    let parsed_since_param = params
        .since
        .clone()
        .unwrap_or_else(|| String::from("0.0"))
        .split('.')
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    let parsed_upto_param = params
        .upto
        .clone()
        .unwrap_or_else(|| String::from("0.0"))
        .split('.')
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    if (params.since.is_some()
        && (parsed_since_param.contains(&0) || parsed_since_param.len() != 2))
        || (params.upto.is_some()
            && (parsed_upto_param.contains(&0) || parsed_upto_param.len() != 2))
    {
        return HttpResponse::BadRequest()
            .status(reqwest::StatusCode::BAD_REQUEST)
            .body("Invalid query parameter(s)");
    }

    let valid_years = HashSet::from(crate::constants::YEARS_LIST);
    let valid_months = HashSet::from(crate::constants::MONTHS_LIST);

    let parsed_since_param = ParsedQueryParam {
        year: parsed_since_param[0],
        month: parsed_since_param[1] as u8,
    };

    let parsed_upto_param = ParsedQueryParam {
        year: parsed_upto_param[0],
        month: parsed_upto_param[1] as u8,
    };

    let since_param_is_valid = valid_years.contains(&parsed_since_param.year)
        && valid_months.contains(&parsed_since_param.month);

    let upto_param_is_valid = valid_years.contains(&parsed_upto_param.year)
        && valid_months.contains(&parsed_upto_param.month);

    if (params.since.is_some() && !since_param_is_valid)
        || (params.upto.is_some() && !upto_param_is_valid)
    {
        return HttpResponse::BadRequest()
            .status(reqwest::StatusCode::BAD_REQUEST)
            .body("Invalid query parameter(s)");
    }

    type HandlerResponse = crate::api_types::HandlerResponse<Vec<ResponseStructure>>;
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
                        month: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                            .unwrap()
                            .format("%Y-%m-%d")
                            .to_string(),
                        positive: daily_item.jumlah_positif.value as i32,
                        recovered: daily_item.jumlah_sembuh.value as i32,
                        deaths: daily_item.jumlah_meninggal.value as i32,
                        active: daily_item.jumlah_dirawat.value as i32,
                    })
                    .filter(|daily| {
                        let parsed_daily_month =
                            NaiveDate::parse_from_str(&daily.month, "%Y-%m-%d").unwrap();

                        if params.since.is_some() && params.upto.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-01",
                                    parsed_since_param.year, parsed_since_param.month
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let upto_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_upto_param.year,
                                    parsed_upto_param.month,
                                    NaiveDate::from_ymd(
                                        parsed_upto_param.year as i32,
                                        parsed_upto_param.month as u32,
                                        1
                                    )
                                    .last_day_of_month()
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let since_compared = parsed_daily_month
                                .signed_duration_since(since_date)
                                .num_days();
                            let upto_compared = parsed_daily_month
                                .signed_duration_since(upto_date)
                                .num_days();

                            return since_compared >= 0 && upto_compared <= 0;
                        }

                        if params.since.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-01",
                                    parsed_since_param.year, parsed_since_param.month
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let since_compared = parsed_daily_month
                                .signed_duration_since(since_date)
                                .num_days();

                            if since_compared < 0 {
                                return false;
                            }

                            return true;
                        }

                        if params.upto.is_some() {
                            let upto_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_upto_param.year,
                                    parsed_upto_param.month,
                                    NaiveDate::from_ymd(
                                        parsed_upto_param.year as i32,
                                        parsed_upto_param.month as u32,
                                        1
                                    )
                                    .last_day_of_month()
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let upto_compared = parsed_daily_month
                                .signed_duration_since(upto_date)
                                .num_days();

                            if upto_compared > 0 {
                                return false;
                            }

                            return true;
                        }

                        true
                    })
                    .collect();

                let mut years_list: Vec<u16> = new_harian
                    .iter()
                    .map(|daily_item| {
                        NaiveDate::parse_from_str(&daily_item.month, "%Y-%m-%d")
                            .unwrap()
                            .year() as u16
                    })
                    .collect::<HashSet<u16>>()
                    .into_iter()
                    .collect();

                years_list.sort_unstable();
                let mut to_return: Vec<ResponseStructure> = Vec::new();

                years_list.iter().for_each(|year| {
                    let cloned_harian: Vec<ResponseStructure> = new_harian.clone();
                    let current_year_harian: Vec<ResponseStructure> = cloned_harian
                        .into_iter()
                        .filter(|daily| {
                            NaiveDate::parse_from_str(&daily.month, "%Y-%m-%d")
                                .unwrap()
                                .year() as u16
                                == *year
                        })
                        .collect();

                    let mut months_list: Vec<u32> = current_year_harian
                        .iter()
                        .map(|daily_item| {
                            NaiveDate::parse_from_str(&daily_item.month, "%Y-%m-%d")
                                .unwrap()
                                .month()
                        })
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect();

                    months_list.sort_unstable();
                    months_list.iter().for_each(|month| {
                        let folded = new_harian
                            .iter()
                            .filter(|daily| {
                                let parsed =
                                    NaiveDate::parse_from_str(&daily.month, "%Y-%m-%d").unwrap();
                                parsed.year().to_string() == *year.to_string()
                                    && parsed.month().to_string() == *month.to_string()
                            })
                            .fold(
                                ResponseStructure {
                                    month: format!("{}-{:0>2}", *year, month),
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
pub async fn specific_year(
    path: web::Path<YearPath>,
    params: web::Query<QueryParams>,
) -> HttpResponse {
    let selected_year = path.year;
    let parsed_since_param = params
        .since
        .clone()
        .unwrap_or_else(|| String::from("0.0"))
        .split('.')
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    let parsed_upto_param = params
        .upto
        .clone()
        .unwrap_or_else(|| String::from("0.0"))
        .split('.')
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    if (params.since.is_some()
        && (parsed_since_param.contains(&0) || parsed_since_param.len() != 2))
        || (params.upto.is_some()
            && (parsed_upto_param.contains(&0) || parsed_upto_param.len() != 2))
    {
        return HttpResponse::BadRequest()
            .status(reqwest::StatusCode::BAD_REQUEST)
            .body("Invalid query parameter(s)");
    }

    let valid_years = HashSet::from(crate::constants::YEARS_LIST);
    let valid_months = HashSet::from(crate::constants::MONTHS_LIST);

    let parsed_since_param = ParsedQueryParam {
        year: parsed_since_param[0],
        month: parsed_since_param[1] as u8,
    };

    let parsed_upto_param = ParsedQueryParam {
        year: parsed_upto_param[0],
        month: parsed_upto_param[1] as u8,
    };

    let since_param_is_valid = parsed_since_param.year == selected_year
        && valid_years.contains(&parsed_since_param.year)
        && valid_months.contains(&parsed_since_param.month);

    let upto_param_is_valid = parsed_upto_param.year == selected_year
        && valid_years.contains(&parsed_upto_param.year)
        && valid_months.contains(&parsed_upto_param.month);

    if (params.since.is_some() && !since_param_is_valid)
        || (params.upto.is_some() && !upto_param_is_valid)
    {
        return HttpResponse::BadRequest()
            .status(reqwest::StatusCode::BAD_REQUEST)
            .body("Invalid query parameter(s)");
    }

    type HandlerResponse = crate::api_types::HandlerResponse<Vec<ResponseStructure>>;
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
                    .filter(|daily_item| {
                        DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                            .unwrap()
                            .year()
                            == selected_year as i32
                    })
                    .map(|daily_item| ResponseStructure {
                        month: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                            .unwrap()
                            .format("%Y-%m-%d")
                            .to_string(),
                        positive: daily_item.jumlah_positif.value as i32,
                        recovered: daily_item.jumlah_sembuh.value as i32,
                        deaths: daily_item.jumlah_meninggal.value as i32,
                        active: daily_item.jumlah_dirawat.value as i32,
                    })
                    .filter(|daily| {
                        let parsed_daily_month =
                            NaiveDate::parse_from_str(&daily.month, "%Y-%m-%d").unwrap();

                        if params.since.is_some() && params.upto.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-01",
                                    parsed_since_param.year, parsed_since_param.month
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let upto_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_upto_param.year,
                                    parsed_upto_param.month,
                                    NaiveDate::from_ymd(
                                        parsed_upto_param.year as i32,
                                        parsed_upto_param.month as u32,
                                        1
                                    )
                                    .last_day_of_month()
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let since_compared = parsed_daily_month
                                .signed_duration_since(since_date)
                                .num_days();
                            let upto_compared = parsed_daily_month
                                .signed_duration_since(upto_date)
                                .num_days();

                            return since_compared >= 0 && upto_compared <= 0;
                        }

                        if params.since.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-01",
                                    parsed_since_param.year, parsed_since_param.month
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let since_compared = parsed_daily_month
                                .signed_duration_since(since_date)
                                .num_days();

                            if since_compared < 0 {
                                return false;
                            }

                            return true;
                        }

                        if params.upto.is_some() {
                            let upto_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_upto_param.year,
                                    parsed_upto_param.month,
                                    NaiveDate::from_ymd(
                                        parsed_upto_param.year as i32,
                                        parsed_upto_param.month as u32,
                                        1
                                    )
                                    .last_day_of_month()
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let upto_compared = parsed_daily_month
                                .signed_duration_since(upto_date)
                                .num_days();

                            if upto_compared > 0 {
                                return false;
                            }

                            return true;
                        }

                        true
                    })
                    .collect();

                let mut months_list: Vec<u32> = new_harian
                    .iter()
                    .map(|daily_item| {
                        NaiveDate::parse_from_str(&daily_item.month, "%Y-%m-%d")
                            .unwrap()
                            .month()
                    })
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();

                months_list.sort_unstable();
                let mut to_return: Vec<ResponseStructure> = Vec::new();
                months_list.iter().for_each(|month| {
                    let folded = new_harian
                        .iter()
                        .filter(|daily| {
                            let parsed =
                                NaiveDate::parse_from_str(&daily.month, "%Y-%m-%d").unwrap();
                            parsed.month().to_string() == *month.to_string()
                        })
                        .fold(
                            ResponseStructure {
                                month: format!("{}-{:0>2}", selected_year, month),
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

#[get("/{year}/{month}")]
pub async fn specific_month(path: web::Path<YearMonthPath>) -> HttpResponse {
    type HandlerResponse = crate::api_types::HandlerResponse<ResponseStructure>;

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
                        let parsed_date =
                            DateTime::parse_from_rfc3339(&daily_item.key_as_string).unwrap();

                        parsed_date.year() == selected_year as i32
                            && parsed_date.month() == selected_month as u32
                    })
                    .map(|daily_item| ResponseStructure {
                        month: DateTime::parse_from_rfc3339(&daily_item.key_as_string)
                            .unwrap()
                            .format("%Y-%m-%d")
                            .to_string(),
                        positive: daily_item.jumlah_positif.value as i32,
                        recovered: daily_item.jumlah_sembuh.value as i32,
                        deaths: daily_item.jumlah_meninggal.value as i32,
                        active: daily_item.jumlah_dirawat.value as i32,
                    })
                    .collect();

                let to_return = new_harian
                    .iter()
                    .filter(|daily| {
                        let parsed = NaiveDate::parse_from_str(&daily.month, "%Y-%m-%d").unwrap();
                        parsed.month().to_string() == selected_month.to_string()
                    })
                    .fold(
                        ResponseStructure {
                            month: format!("{}-{:0>2}", selected_year, selected_month),
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
