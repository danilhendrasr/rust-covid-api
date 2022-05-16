use actix_web::{web, App, HttpServer};
use nodeflux_assignment::routes;

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
                    .service(routes::monthly::index)
                    .service(routes::monthly::specific_year)
                    .service(routes::monthly::specific_month),
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
