use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

static COVID_API_ENDPOINT: &'static str = "https://data.covid19.go.id/public/api/update.json";
static VACCINATION_API_ENDPOINT: &'static str =
    "https://data.covid19.go.id/public/api/pemeriksaan-vaksinasi.json";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index_handler))
            .service(
                web::scope("/yearly")
                    .service(yearly::index)
                    .service(yearly::specific_year),
            )
            .service(
                web::scope("/monthly")
                    .service(monthly::index)
                    .service(monthly::specific_year)
                    .service(monthly::specific_month),
            )
            .service(
                web::scope("/daily")
                    .service(daily::index)
                    .service(daily::specific_year)
                    .service(daily::specific_month)
                    .service(daily::specific_date),
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}

async fn index_handler() -> HttpResponse {
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
    struct Penambahan {
        jumlah_positif: u32,
        jumlah_meninggal: u32,
        jumlah_sembuh: u32,
        jumlah_dirawat: u32,
        tanggal: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Total {
        jumlah_positif: u32,
        jumlah_dirawat: u32,
        jumlah_sembuh: u32,
        jumlah_meninggal: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct WantedObject {
        penambahan: Penambahan,
        total: Total,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CovidAPIResponse {
        update: WantedObject,
    }

    let resp = reqwest::get(COVID_API_ENDPOINT).await;

    match resp {
        Err(_) => HttpResponse::InternalServerError()
            .body("Could not get the data, retry in a few minutes"),
        Ok(x) => match x.json::<CovidAPIResponse>().await {
            Err(_) => HttpResponse::InternalServerError()
                .body("There's something wrong with us, hang tight"),
            Ok(y) => {
                let to_return: HandlerResponse = HandlerResponse {
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

mod yearly {
    use actix_web::{get, HttpRequest, Result};

    #[get("")]
    pub async fn index(_req: HttpRequest) -> Result<String> {
        Ok(String::from("yearly"))
    }

    #[get("/{year}")]
    pub async fn specific_year(_req: HttpRequest) -> Result<String> {
        Ok(String::from("yearly/{year}"))
    }
}

mod monthly {
    use actix_web::{get, HttpRequest, Result};

    #[get("")]
    pub async fn index(_req: HttpRequest) -> Result<String> {
        Ok(String::from("monthly"))
    }

    #[get("/{year}")]
    pub async fn specific_year(_req: HttpRequest) -> Result<String> {
        Ok(String::from("monthly/{year}"))
    }

    #[get("/{year}/{month}")]
    pub async fn specific_month(_req: HttpRequest) -> Result<String> {
        Ok(String::from("monthly/{year}/{month}"))
    }
}

mod daily {
    use actix_web::{get, HttpRequest, Result};

    #[get("")]
    pub async fn index(_req: HttpRequest) -> Result<String> {
        Ok(String::from("daily"))
    }

    #[get("/{year}")]
    pub async fn specific_year(_req: HttpRequest) -> Result<String> {
        Ok(String::from("daily/{year}"))
    }

    #[get("/{year}/{month}")]
    pub async fn specific_month(_req: HttpRequest) -> Result<String> {
        Ok(String::from("daily/{year}/{month}"))
    }

    #[get("/{year}/{month}/{date}")]
    pub async fn specific_date(_req: HttpRequest) -> Result<String> {
        Ok(String::from("daily/{year}/{month}/{date}"))
    }
}
