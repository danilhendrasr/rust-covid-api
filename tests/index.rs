use actix_web::{test, web, App};
use nodeflux_assignment::routes;

#[actix_web::test]
async fn returns_200() {
    let app =
        test::init_service(App::new().route("/", web::get().to(routes::index::index_handler)))
            .await;

    let req = test::TestRequest::with_uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 200);
}
