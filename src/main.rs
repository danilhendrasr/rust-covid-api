use std::io::ErrorKind;

use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use rust_covid_api::routes::{self, daily, monthly};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8082))
        .map_err(|_| std::io::Error::new(ErrorKind::InvalidInput, "Invalid port configuration."))?;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index::index_handler))
            .service(
                web::scope("/yearly")
                    .service(routes::yearly::index_handler)
                    .service(routes::yearly::specific_year),
            )
            .service(
                web::scope("/monthly")
                    .wrap(from_fn(monthly::middleware::filter_malformed_query_params))
                    .service(routes::monthly::index_handler)
                    .service(routes::monthly::specific_year)
                    .service(routes::monthly::specific_month),
            )
            .service(
                web::scope("/daily")
                    .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                    .service(routes::daily::index_handler)
                    .service(routes::daily::specific_year)
                    .service(routes::daily::specific_month)
                    .service(routes::daily::specific_day),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
