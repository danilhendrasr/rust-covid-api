use actix_web::{test, web, App};
use rust_covid_api::routes;

#[cfg(test)]
mod all_years {
    use super::*;

    #[actix_web::test]
    async fn returns_200() {
        let app = test::init_service(
            App::new().service(web::scope("/yearly").service(routes::yearly::index_handler)),
        )
        .await;

        let req = test::TestRequest::with_uri("/yearly").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
    }
}

#[cfg(test)]
mod specific_year {
    use super::*;

    #[actix_web::test]
    async fn returns_200_given_valid_year() {
        let app = test::init_service(
            App::new().service(web::scope("/yearly").service(routes::yearly::specific_year)),
        )
        .await;

        let req = test::TestRequest::with_uri("/yearly/2020").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_year() {
        let app = test::init_service(
            App::new().service(web::scope("/yearly").service(routes::yearly::specific_year)),
        )
        .await;

        let req = test::TestRequest::with_uri("/yearly/2018").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}
