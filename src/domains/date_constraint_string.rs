use crate::utils::*;

#[derive(Debug, PartialEq)]
pub struct DateConstraintString(String);

impl DateConstraintString {
    pub fn parse(value: String) -> Result<Self, String> {
        if !value.contains(".") {
            match is_valid_year_str(&value) {
                true => return Ok(DateConstraintString(value)),
                false => return Err(value),
            }
        }

        let splitted_string = value.split(".").collect::<Vec<&str>>();
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

    pub fn content(&self) -> Vec<u32> {
        let content = &self.0;
        if !content.contains(".") {
            // The value is guaranteed to be valid, thus we can unwrap safely
            return vec![parse_date_part_str(content).unwrap()];
        }

        let splitted_content = content
            .split(".")
            .into_iter()
            .map(|date_part| parse_date_part_str(date_part).unwrap())
            .collect::<Vec<u32>>();

        splitted_content
    }
}
