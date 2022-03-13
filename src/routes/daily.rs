use actix_web::{get, web, HttpResponse};
use chrono::prelude::*;
use chrono_utilities::naive::DateTransitions;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Debug)]
struct ResponseStructure {
    date: String,
    positive: i32,
    recovered: i32,
    deaths: i32,
    active: i32,
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
    date: u8,
}

type HandlerResponse = crate::api_types::HandlerResponse<Vec<ResponseStructure>>;

#[derive(Serialize, Deserialize, Debug)]
struct Update {
    harian: Vec<crate::api_types::Harian>,
}

#[get("")]
pub async fn index(params: web::Query<QueryParams>) -> HttpResponse {
    let parsed_since_param = params
        .since
        .clone()
        .unwrap_or(String::from("0.0.0"))
        .split(".")
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    let parsed_upto_param = params
        .upto
        .clone()
        .unwrap_or(String::from("0.0.0"))
        .split(".")
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    if (params.since.is_some()
        && (parsed_since_param.contains(&0) || parsed_since_param.len() != 3))
        || (params.upto.is_some()
            && (parsed_upto_param.contains(&0) || parsed_upto_param.len() != 3))
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
        date: parsed_since_param[2] as u8,
    };

    let parsed_upto_param = ParsedQueryParam {
        year: parsed_upto_param[0],
        month: parsed_upto_param[1] as u8,
        date: parsed_upto_param[2] as u8,
    };

    let since_param_is_valid = valid_years.contains(&parsed_since_param.year)
        && valid_months.contains(&parsed_since_param.month)
        && (parsed_since_param.date >= 1
            && parsed_since_param.date
                <= NaiveDate::from_ymd(
                    parsed_since_param.year as i32,
                    parsed_since_param.month as u32,
                    1,
                )
                .last_day_of_month() as u8);

    let upto_param_is_valid = valid_years.contains(&parsed_upto_param.year)
        && valid_months.contains(&parsed_upto_param.month)
        && (parsed_upto_param.date >= 1
            && parsed_upto_param.date
                <= NaiveDate::from_ymd(
                    parsed_upto_param.year as i32,
                    parsed_upto_param.month as u32,
                    1,
                )
                .last_day_of_month() as u8);

    if (params.since.is_some() && !since_param_is_valid)
        || (params.upto.is_some() && !upto_param_is_valid)
    {
        return HttpResponse::BadRequest()
            .status(reqwest::StatusCode::BAD_REQUEST)
            .body("Invalid query parameter(s)");
    }

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

                let new_harian = new_harian
                    .into_iter()
                    .filter(|daily| {
                        let parsed_daily_month =
                            NaiveDate::parse_from_str(&daily.date, "%Y-%m-%d").unwrap();

                        if params.since.is_some() && params.upto.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_since_param.year,
                                    parsed_since_param.month,
                                    parsed_since_param.date
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let upto_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_upto_param.year,
                                    parsed_upto_param.month,
                                    parsed_upto_param.date
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

                            if since_compared >= 0 && upto_compared <= 0 {
                                return true;
                            } else {
                                return false;
                            };
                        }

                        if params.since.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_since_param.year,
                                    parsed_since_param.month,
                                    parsed_since_param.date
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
                                    parsed_upto_param.date
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
pub async fn specific_year(
    path: web::Path<YearPath>,
    params: web::Query<QueryParams>,
) -> HttpResponse {
    let selected_year = path.year;
    let parsed_since_param = params
        .since
        .clone()
        .unwrap_or(String::from("0.0.0"))
        .split(".")
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    let parsed_upto_param = params
        .upto
        .clone()
        .unwrap_or(String::from("0.0.0"))
        .split(".")
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    if (params.since.is_some()
        && (parsed_since_param.contains(&0) || parsed_since_param.len() != 3))
        || (params.upto.is_some()
            && (parsed_upto_param.contains(&0) || parsed_upto_param.len() != 3))
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
        date: parsed_since_param[2] as u8,
    };

    let parsed_upto_param = ParsedQueryParam {
        year: parsed_upto_param[0],
        month: parsed_upto_param[1] as u8,
        date: parsed_upto_param[2] as u8,
    };

    let since_param_is_valid = parsed_since_param.year == selected_year
        && valid_years.contains(&parsed_since_param.year)
        && valid_months.contains(&parsed_since_param.month)
        && (parsed_since_param.date >= 1
            && parsed_since_param.date
                <= NaiveDate::from_ymd(
                    parsed_since_param.year as i32,
                    parsed_since_param.month as u32,
                    1,
                )
                .last_day_of_month() as u8);

    let upto_param_is_valid = parsed_upto_param.year == selected_year
        && valid_years.contains(&parsed_upto_param.year)
        && valid_months.contains(&parsed_upto_param.month)
        && (parsed_upto_param.date >= 1
            && parsed_upto_param.date
                <= NaiveDate::from_ymd(
                    parsed_upto_param.year as i32,
                    parsed_upto_param.month as u32,
                    1,
                )
                .last_day_of_month() as u8);

    if (params.since.is_some() && !since_param_is_valid)
        || (params.upto.is_some() && !upto_param_is_valid)
    {
        return HttpResponse::BadRequest()
            .status(reqwest::StatusCode::BAD_REQUEST)
            .body("Invalid query parameter(s)");
    }

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

                let new_harian = new_harian
                    .into_iter()
                    .filter(|daily| {
                        let parsed_daily_month =
                            NaiveDate::parse_from_str(&daily.date, "%Y-%m-%d").unwrap();

                        if params.since.is_some() && params.upto.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_since_param.year,
                                    parsed_since_param.month,
                                    parsed_since_param.date
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let upto_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_upto_param.year,
                                    parsed_upto_param.month,
                                    parsed_upto_param.date
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

                            if since_compared >= 0 && upto_compared <= 0 {
                                return true;
                            } else {
                                return false;
                            };
                        }

                        if params.since.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_since_param.year,
                                    parsed_since_param.month,
                                    parsed_since_param.date
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
                                    parsed_upto_param.date
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
pub async fn specific_month(
    path: web::Path<YearMonthPath>,
    params: web::Query<QueryParams>,
) -> HttpResponse {
    let selected_year = path.year;
    let selected_month = path.month;

    let parsed_since_param = params
        .since
        .clone()
        .unwrap_or(String::from("0.0.0"))
        .split(".")
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    let parsed_upto_param = params
        .upto
        .clone()
        .unwrap_or(String::from("0.0.0"))
        .split(".")
        .map(|x| x.parse::<u16>().unwrap_or(0))
        .collect::<Vec<u16>>();

    if (params.since.is_some()
        && (parsed_since_param.contains(&0) || parsed_since_param.len() != 3))
        || (params.upto.is_some()
            && (parsed_upto_param.contains(&0) || parsed_upto_param.len() != 3))
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
        date: parsed_since_param[2] as u8,
    };

    let parsed_upto_param = ParsedQueryParam {
        year: parsed_upto_param[0],
        month: parsed_upto_param[1] as u8,
        date: parsed_upto_param[2] as u8,
    };

    let since_param_is_valid = parsed_since_param.year == selected_year
        && valid_years.contains(&parsed_since_param.year)
        && parsed_since_param.month == selected_month
        && valid_months.contains(&parsed_since_param.month)
        && (parsed_since_param.date >= 1
            && parsed_since_param.date
                <= NaiveDate::from_ymd(
                    parsed_since_param.year as i32,
                    parsed_since_param.month as u32,
                    1,
                )
                .last_day_of_month() as u8);

    let upto_param_is_valid = parsed_upto_param.year == selected_year
        && valid_years.contains(&parsed_upto_param.year)
        && parsed_upto_param.month == selected_month
        && valid_months.contains(&parsed_upto_param.month)
        && (parsed_upto_param.date >= 1
            && parsed_upto_param.date
                <= NaiveDate::from_ymd(
                    parsed_upto_param.year as i32,
                    parsed_upto_param.month as u32,
                    1,
                )
                .last_day_of_month() as u8);

    if (params.since.is_some() && !since_param_is_valid)
        || (params.upto.is_some() && !upto_param_is_valid)
    {
        return HttpResponse::BadRequest()
            .status(reqwest::StatusCode::BAD_REQUEST)
            .body("Invalid query parameter(s)");
    }

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

                let new_harian = new_harian
                    .into_iter()
                    .filter(|daily| {
                        let parsed_daily_month =
                            NaiveDate::parse_from_str(&daily.date, "%Y-%m-%d").unwrap();

                        if params.since.is_some() && params.upto.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_since_param.year,
                                    parsed_since_param.month,
                                    parsed_since_param.date
                                ),
                                "%Y-%m-%d",
                            )
                            .unwrap();

                            let upto_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_upto_param.year,
                                    parsed_upto_param.month,
                                    parsed_upto_param.date
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

                            if since_compared >= 0 && upto_compared <= 0 {
                                return true;
                            } else {
                                return false;
                            };
                        }

                        if params.since.is_some() {
                            let since_date = NaiveDate::parse_from_str(
                                &format!(
                                    "{}-{:0>2}-{}",
                                    parsed_since_param.year,
                                    parsed_since_param.month,
                                    parsed_since_param.date
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
                                    parsed_upto_param.date
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
