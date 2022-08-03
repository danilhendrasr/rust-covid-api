use std::collections::HashSet;

pub struct Daily(pub Vec<DailyItem>);
pub struct Monthly(pub Vec<MonthlyItem>);
pub struct Yearly(pub Vec<YearlyItem>);

impl Daily {
    pub fn get_all_days_in_a_year(self, year: i32) -> Result<Daily, String> {
        let filtered = self
            .0
            .into_iter()
            .filter(|daily| daily.year == year)
            .collect::<Vec<DailyItem>>();

        if filtered.is_empty() {
            return Err("Year not found".into());
        }

        Ok(Daily(filtered))
    }

    pub fn get_all_daily_cases_in_a_month(self, year: i32, month: i32) -> Result<Daily, String> {
        let filtered = self
            .get_all_days_in_a_year(year)?
            .0
            .into_iter()
            .filter(|daily| daily.month == month as u32)
            .collect::<Vec<DailyItem>>();

        if filtered.is_empty() {
            return Err("Month not found".into());
        }

        Ok(Daily(filtered))
    }

    pub fn get_specific_day(self, year: i32, month: i32, day: i32) -> Result<DailyItem, String> {
        let filtered = self
            .get_all_daily_cases_in_a_month(year, month)?
            .0
            .into_iter()
            .find(|daily| daily.day == day as u32);

        match filtered {
            Some(daily_case) => Ok(daily_case),
            None => Err("Day not found".into()),
        }
    }

    /// Get distinct months from all daily cases in a year.<br>
    /// **Output**: `[10, 11, 12]`
    fn get_distinct_months(&self, year: &i32) -> Vec<u32> {
        let mut distinct_months = self
            .0
            .iter()
            .filter(|daily_item| daily_item.year == *year)
            .map(|daily_item| daily_item.month)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        distinct_months.sort_unstable();
        distinct_months
    }

    pub fn to_monthly(&self) -> Monthly {
        let years_list = self.get_distinct_years();

        let mut to_return: Vec<MonthlyItem> = Vec::new();
        years_list.iter().for_each(|current_year| {
            let months_list = self.get_distinct_months(current_year);
            months_list.iter().for_each(|current_month| {
                let folded = self
                    .0
                    .iter()
                    .filter(|daily| daily.year == *current_year && daily.month == *current_month)
                    .fold(
                        MonthlyItem {
                            year: *current_year,
                            month: *current_month,
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
            })
        });

        Monthly(to_return)
    }

    pub fn get_all_months_in_a_year(&self, year: i32) -> Result<Monthly, String> {
        let filtered = self
            .to_monthly()
            .0
            .into_iter()
            .filter(|e| e.year == year)
            .collect::<Vec<MonthlyItem>>();

        if filtered.is_empty() {
            return Err("Year not found".into());
        }

        Ok(Monthly(filtered))
    }

    pub fn get_specific_month(self, year: i32, month: i32) -> Result<MonthlyItem, String> {
        match self
            .get_all_months_in_a_year(year)?
            .0
            .into_iter()
            .find(|x| x.month == month as u32)
        {
            Some(value) => Ok(value),
            None => Err("Month not found".into()),
        }
    }

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
        match self.to_yearly().0.into_iter().find(|e| e.year == year) {
            Some(value) => Ok(value),
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DailyItem {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub positive: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub active: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MonthlyItem {
    pub year: i32,
    pub month: u32,
    pub positive: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub active: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct YearlyItem {
    pub year: i32,
    pub positive: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub active: i32,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub since: Option<String>,
    pub upto: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct YearPath {
    pub year: i32,
}

#[derive(serde::Deserialize)]
pub struct YearMonthPath {
    pub year: i32,
    pub month: i32,
}

#[derive(serde::Deserialize)]
pub struct YearMonthDayPath {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

pub mod source_api {
    use super::{Daily, DailyItem};
    use chrono::{DateTime, Datelike};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SourceAPIResponse {
        pub update: Update,
    }

    impl SourceAPIResponse {
        /// Convert daily cases JSON structure from source API into our own daily format.
        /// ### From
        /// ```json
        /// {
        ///     "key_as_string": "2020-03-02T00:00:00.000Z",
        ///     "key": 1583107200000,
        ///     "doc_count": 1,
        ///     "jumlah_meninggal": {
        ///         "value": 0
        ///     },
        ///     "jumlah_sembuh": {
        ///         "value": 0
        ///     },
        ///     "jumlah_positif": {
        ///         "value": 2
        ///     },
        ///     "jumlah_dirawat": {
        ///         "value": 2
        ///     },
        ///     "jumlah_positif_kum": {
        ///         "value": 2
        ///     },
        ///     "jumlah_sembuh_kum": {
        ///         "value": 0
        ///     },
        ///     "jumlah_meninggal_kum": {
        ///         "value": 0
        ///     },
        ///     "jumlah_dirawat_kum": {
        ///         "value": 2
        ///     }
        /// }
        /// ```
        /// ### To
        /// ```json
        /// {
        ///     "year": 2020,
        ///     "month": 3,
        ///     "date": 02,
        ///     "positive": 2,
        ///     "recovered": 0,
        ///     "deaths": 0,
        ///     "active": 2
        /// }
        /// ```
        pub fn to_daily(&self) -> Daily {
            self.update
                .harian
                .iter()
                .map(|source_daily_case| {
                    let parsed_case_key =
                        DateTime::parse_from_rfc3339(&source_daily_case.key_as_string).unwrap();

                    DailyItem {
                        year: parsed_case_key.year(),
                        month: parsed_case_key.month(),
                        day: parsed_case_key.day(),
                        positive: source_daily_case.jumlah_positif.value,
                        recovered: source_daily_case.jumlah_sembuh.value,
                        deaths: source_daily_case.jumlah_meninggal.value,
                        active: source_daily_case.jumlah_dirawat.value,
                    }
                })
                .collect()
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Update {
        pub harian: Vec<Harian>,
        pub total: Total,
        pub penambahan: Penambahan,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Harian {
        pub key_as_string: String,
        pub key: u64,
        pub doc_count: u32,
        pub jumlah_meninggal: HarianKeyValue,
        pub jumlah_sembuh: HarianKeyValue,
        pub jumlah_positif: HarianKeyValue,
        pub jumlah_dirawat: HarianKeyValue,
        pub jumlah_positif_kum: HarianKeyValue,
        pub jumlah_sembuh_kum: HarianKeyValue,
        pub jumlah_meninggal_kum: HarianKeyValue,
        pub jumlah_dirawat_kum: HarianKeyValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HarianKeyValue {
        pub value: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Total {
        pub jumlah_positif: u32,
        pub jumlah_dirawat: u32,
        pub jumlah_sembuh: u32,
        pub jumlah_meninggal: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Penambahan {
        pub jumlah_positif: i64,
        pub jumlah_meninggal: i64,
        pub jumlah_sembuh: i64,
        pub jumlah_dirawat: i64,
        pub tanggal: String,
        pub created: String,
    }
}
