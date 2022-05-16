use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Debug)]
pub struct HandlerResponseTemplate<T> {
    pub ok: bool,
    pub data: T,
    pub message: String,
}

pub struct Daily(pub Vec<DailyItem>);
pub struct Yearly(pub Vec<YearlyItem>);

impl Daily {
    /// Get distinct years from all daily cases.<br>
    /// **Output**: `[2019, 2020, 2021, 2022]`
    fn get_distinct_years(&self) -> Vec<i32> {
        let mut distinct_years = self
            .0
            .iter()
            .map(|daily_item| daily_item.year)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        distinct_years.sort_unstable();
        distinct_years
    }

    /// Convert daily into yearly format.
    ///
    /// #### Output
    /// ```json
    /// [
    ///     {
    ///         "year": 2020,
    ///         "positive": 743198,
    ///         "recovered": 611097,
    ///         "deaths": 22138,
    ///         "active": 109963
    ///     },
    ///     {
    ///         "year": 2021,
    ///         "positive": 3519522,
    ///         "recovered": 3503237,
    ///         "deaths": 121956,
    ///         "active": -105671
    ///     },
    ///     ...
    /// ]
    /// ```
    pub fn to_yearly(&self) -> Yearly {
        let years_list = self.get_distinct_years();

        let mut to_return: Vec<YearlyItem> = Vec::new();
        years_list.iter().for_each(|year| {
            let folded = self.0.iter().filter(|daily| daily.year == *year).fold(
                YearlyItem {
                    year: *year,
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

        Yearly(to_return)
    }

    /// Aggregate daily into yearly format and pick 1 specific year.
    ///
    /// #### Output
    /// ```json
    /// {
    ///     "year": 2020,
    ///     "positive": 743198,
    ///     "recovered": 611097,
    ///     "deaths": 22138,
    ///     "active": 109963
    /// }
    /// ```
    pub fn to_specific_yearly(&self, year: i32) -> Result<YearlyItem, String> {
        match self.to_yearly().0.iter().find(|e| e.year == year) {
            Some(value) => Ok((*value).clone()),
            None => Err("Year not found".into()),
        }
    }
}

impl FromIterator<DailyItem> for Daily {
    fn from_iter<T: IntoIterator<Item = DailyItem>>(iter: T) -> Self {
        let mut holder: Vec<DailyItem> = Vec::new();

        for i in iter {
            holder.push(i);
        }

        Self(holder)
    }
}

#[derive(serde::Serialize)]
pub struct DailyItem {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub positive: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub active: i32,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct YearlyItem {
    pub year: i32,
    pub positive: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub active: i32,
}
