use std::io::ErrorKind;

use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use rust_covid_api::{
    api_doc::ApiDoc,
    routes::{self, daily, monthly},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<(), impl std::error::Error> {
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8082))
        .map_err(|_| std::io::Error::new(ErrorKind::InvalidInput, "Invalid port configuration."))?;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(routes::index::daily_cases_summary))
            .route("/health", web::get().to(routes::health::service_health))
            .service(
                web::scope("/yearly")
                    .service(routes::yearly::all_years)
                    .service(routes::yearly::specific_year),
            )
            .service(
                web::scope("/monthly")
                    .wrap(from_fn(monthly::middleware::filter_malformed_query_params))
                    .service(routes::monthly::all_months)
                    .service(routes::monthly::all_months_in_a_year)
                    .service(routes::monthly::specific_month),
            )
            .service(
                web::scope("/daily")
                    .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                    .service(routes::daily::all_days)
                    .service(routes::daily::all_days_in_a_year)
                    .service(routes::daily::all_days_in_a_month)
                    .service(routes::daily::specific_day),
            )
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
