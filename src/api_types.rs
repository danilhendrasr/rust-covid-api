use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Total {
  pub jumlah_positif: u32,
  pub jumlah_dirawat: u32,
  pub jumlah_sembuh: u32,
  pub jumlah_meninggal: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Penambahan {
  pub jumlah_positif: u32,
  pub jumlah_meninggal: u32,
  pub jumlah_sembuh: u32,
  pub jumlah_dirawat: u32,
  pub tanggal: String,
}

#[derive(Serialize, Debug)]
pub struct HandlerResponse<T> {
  pub ok: bool,
  pub data: T,
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarianItemValue {
  pub value: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Harian {
  pub key_as_string: String,
  pub key: u64,
  pub doc_count: u32,
  pub jumlah_meninggal: HarianItemValue,
  pub jumlah_sembuh: HarianItemValue,
  pub jumlah_positif: HarianItemValue,
  pub jumlah_dirawat: HarianItemValue,
  pub jumlah_positif_kum: HarianItemValue,
  pub jumlah_sembuh_kum: HarianItemValue,
  pub jumlah_meninggal_kum: HarianItemValue,
  pub jumlah_dirawat_kum: HarianItemValue,
}
