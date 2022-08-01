use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use nodeflux_assignment::routes::{self, daily, monthly};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
                    .service(routes::daily::specific_month),
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
