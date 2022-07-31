use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use nodeflux_assignment::routes::{self, monthly::middleware::filter_malformed_query_params};

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
                    .wrap(from_fn(filter_malformed_query_params))
                    .service(routes::monthly::index_handler),
            )
            .service(
                web::scope("/daily")
                    .service(routes::daily::index)
                    .service(routes::daily::specific_year)
                    .service(routes::daily::specific_month)
                    .service(routes::daily::specific_date),
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
