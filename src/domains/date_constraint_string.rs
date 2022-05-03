use crate::utils::*;

#[derive(Debug, PartialEq)]
pub struct DateConstraintString(String);

impl DateConstraintString {
    pub fn parse(value: String) -> Result<Self, String> {
        if !value.contains('.') {
            match is_valid_year_str(&value) {
                true => return Ok(DateConstraintString(value)),
                false => return Err(value),
            }
        }

        let splitted_string = value.split('.').collect::<Vec<&str>>();
        if splitted_string.len() < 2 || splitted_string.len() > 3 {
            return Err(value);
        }

        if splitted_string.len() == 2 && (!is_valid_year_month_param(&splitted_string)) {
            return Err(value);
        }

        if splitted_string.len() == 3 && (!is_valid_date_param(&splitted_string)) {
            return Err(value);
        }

        Ok(DateConstraintString(value))
    }
}
