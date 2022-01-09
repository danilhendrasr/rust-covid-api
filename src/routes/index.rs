use crate::{api_types, constants};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub async fn index_handler() -> HttpResponse {
  #[derive(Serialize, Debug)]
  struct HandlerResponseMainData {
    total_positive: u32,
    total_recovered: u32,
    total_deaths: u32,
    total_active: u32,
    new_positive: u32,
    new_recovered: u32,
    new_deaths: u32,
    new_active: u32,
  }

  #[derive(Serialize, Debug)]
  struct HandlerResponse {
    ok: bool,
    data: HandlerResponseMainData,
    message: String,
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct WantedObject {
    penambahan: api_types::Penambahan,
    total: api_types::Total,
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct CovidAPIResponse {
    update: WantedObject,
  }

  let resp = reqwest::get(constants::COVID_API_ENDPOINT).await;

  match resp {
    Err(_) => {
      HttpResponse::InternalServerError().body("Could not get the data, retry in a few minutes")
    }
    Ok(x) => match x.json::<CovidAPIResponse>().await {
      Err(_) => {
        HttpResponse::InternalServerError().body("There's something wrong with us, hang tight")
      }
      Ok(y) => {
        let to_return: api_types::HandlerResponse<HandlerResponseMainData> =
          api_types::HandlerResponse::<HandlerResponseMainData> {
            ok: true,
            data: HandlerResponseMainData {
              total_positive: y.update.total.jumlah_positif,
              total_recovered: y.update.total.jumlah_sembuh,
              total_deaths: y.update.total.jumlah_meninggal,
              total_active: y.update.total.jumlah_dirawat,
              new_positive: y.update.penambahan.jumlah_positif,
              new_recovered: y.update.penambahan.jumlah_sembuh,
              new_deaths: y.update.penambahan.jumlah_meninggal,
              new_active: y.update.penambahan.jumlah_dirawat,
            },
            message: String::from("success"),
          };

        HttpResponse::Ok().body(serde_json::to_string(&to_return).unwrap())
      }
    },
  }
}
