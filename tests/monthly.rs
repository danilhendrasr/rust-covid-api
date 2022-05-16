use actix_web::{test, web, App};
use nodeflux_assignment::routes;

mod all_months {
    use super::*;

    #[actix_web::test]
    async fn returns_200() {
        let app = test::init_service(
            App::new().service(web::scope("/monthly").service(routes::monthly::index)),
        )
        .await;

        let req = test::TestRequest::with_uri("/monthly").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
    }
}

mod all_months_in_year {
    use super::*;

    #[actix_web::test]
    async fn returns_200_given_valid_year() {
        let app = test::init_service(
            App::new().service(web::scope("/monthly").service(routes::monthly::specific_year)),
        )
        .await;

        let req = test::TestRequest::with_uri("/monthly/2020").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_year() {
        let app = test::init_service(
            App::new().service(web::scope("/monthly").service(routes::monthly::specific_year)),
        )
        .await;

        let req = test::TestRequest::with_uri("/monthly/2018").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}

mod specific_month {
    use super::*;

    #[actix_web::test]
    async fn returns_200_given_valid_month() {
        let app = test::init_service(
            App::new().service(web::scope("/monthly").service(routes::monthly::specific_month)),
        )
        .await;

        let req = test::TestRequest::with_uri("/monthly/2021/7").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_month() {
        let app = test::init_service(
            App::new().service(web::scope("/monthly").service(routes::monthly::specific_month)),
        )
        .await;

        let req = test::TestRequest::with_uri("/monthly/2021/13").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}
