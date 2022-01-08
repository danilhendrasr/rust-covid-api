use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| HttpResponse::Ok().content_type("text/plain").body("index")),
            )
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
    .bind("127.0.0.1:8081")?
    .run()
    .await
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
