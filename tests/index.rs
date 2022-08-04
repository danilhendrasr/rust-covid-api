use actix_web::{test, web, App};
use rust_covid_api::routes::{self, index::CasesSummary};

#[actix_web::test]
async fn has_valid_response_structure() {
    let app = test::init_service(
        App::new().route("/", web::get().to(routes::index::daily_cases_summary)),
    )
    .await;

    let req = test::TestRequest::with_uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(
        resp.headers().get("Content-Type").unwrap(),
        "application/json"
    );
    let _body: CasesSummary = test::read_body_json(resp).await;
}
