use super::crate_own::{Daily, DailyItem};
use chrono::{DateTime, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub update: Update,
}

impl Response {
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
