use crate::{
    constants::{COVID_API_ENDPOINT, MONTHS_LIST, YEARS_LIST},
    types,
};
use chrono::prelude::NaiveDate;
use chrono_utilities::naive::DateTransitions;
use std::num::ParseIntError;

pub fn is_valid_date_param(vec: &[&str]) -> bool {
    is_valid_year_month_param(&vec[..=1]) && is_valid_day_str(vec[0], vec[1], vec[2])
}

pub fn is_valid_day_str(year: &str, month: &str, day: &str) -> bool {
    let parsed_year = match parse_date_part_str(year) {
        Ok(value) => value,
        Err(_) => return false,
    };

    let parsed_month = match parse_date_part_str(month) {
        Ok(value) => value,
        Err(_) => return false,
    };

    let last_day_of_month =
        NaiveDate::from_ymd(parsed_year as i32, parsed_month as u32, 1).last_day_of_month();

    let day = match day.parse::<u32>() {
        Ok(value) => value,
        Err(_) => return false,
    };

    let is_day_valid = day >= 1 && day <= last_day_of_month;

    is_valid_year_month_param(&[year, month]) && is_day_valid
}

pub fn is_valid_year_month_param(input_vec: &[&str]) -> bool {
    is_valid_year_str(input_vec[0]) && is_valid_month_str(input_vec[1])
}

pub fn is_valid_year_str(year: &str) -> bool {
    let year = match parse_date_part_str(year) {
        Ok(value) => value,
        Err(_) => return false,
    };

    YEARS_LIST.contains(&(year as u16))
}

pub fn parse_date_part_str(input: &str) -> Result<u32, ParseIntError> {
    input.parse::<u32>()
}

pub fn is_valid_month_str(month: &str) -> bool {
    let month = match parse_date_part_str(month) {
        Ok(value) => value,
        Err(_) => return false,
    };

    MONTHS_LIST.contains(&(month as u8))
}

pub async fn fetch_data_from_source_api() -> Result<types::source_api::Response, String> {
    let resp = reqwest::get(COVID_API_ENDPOINT)
        .await
        .map_err(|_| "Failed fetching data from source API.")?;

    let json = resp.json().await.map_err(|_| "Failed processing data.")?;

    Ok(json)
}
