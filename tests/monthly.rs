use actix_web::{test, web, App};
use rust_covid_api::{routes::monthly, types::MonthlyCase};

mod all_months {
    use actix_web_lab::middleware::from_fn;
    use chrono::Datelike;

    use super::*;

    #[actix_web::test]
    async fn returns_all_months() {
        let current_time = chrono::Utc::now();
        let current_year = current_time.year();
        let current_month = current_time.month();

        let earliest_year = 2020;
        let earliest_month = 3;

        let app = test::init_service(
            App::new()
                .wrap(from_fn(monthly::middleware::filter_malformed_query_params))
                .service(web::scope("/monthly").service(monthly::all_months)),
        )
        .await;

        let req = test::TestRequest::get().uri("/monthly").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: Vec<MonthlyCase> = test::read_body_json(resp).await;
        assert!(!body.is_empty());
        assert_eq!(body[0].year, earliest_year);
        assert_eq!(body[0].month, earliest_month);

        let last_item = body.last().unwrap();
        assert_eq!(last_item.year, current_year);
        assert_eq!(last_item.month, current_month);
    }
}

mod all_months_in_a_year {
    use actix_web_lab::middleware::from_fn;

    use super::*;

    #[actix_web::test]
    async fn returns_all_months_in_a_year() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(monthly::middleware::filter_malformed_query_params))
                .service(web::scope("/monthly").service(monthly::all_months_in_a_year)),
        )
        .await;

        let chosen_year = 2021;
        let req = test::TestRequest::get()
            .uri(&format!("/monthly/{}", chosen_year))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: Vec<MonthlyCase> = test::read_body_json(resp).await;
        assert!(body.len() == 12);
        assert_eq!(body[0].year, chosen_year);
        assert_eq!(body.last().unwrap().year, chosen_year);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_year() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(monthly::middleware::filter_malformed_query_params))
                .service(web::scope("/monthly").service(monthly::all_months_in_a_year)),
        )
        .await;

        let req = test::TestRequest::get().uri("/monthly/2018").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}

mod specific_month {
    use actix_web_lab::middleware::from_fn;

    use super::*;

    #[actix_web::test]
    async fn returns_correct_data_given_valid_month() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(monthly::middleware::filter_malformed_query_params))
                .service(web::scope("/monthly").service(monthly::specific_month)),
        )
        .await;

        let chosen_year = 2020;
        let chosen_month = 10;
        let req = test::TestRequest::get()
            .uri(&format!("/monthly/{chosen_year}/{chosen_month}"))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: MonthlyCase = test::read_body_json(resp).await;
        assert_eq!(body.year, chosen_year);
        assert_eq!(body.month, chosen_month);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_month() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(monthly::middleware::filter_malformed_query_params))
                .service(web::scope("/monthly").service(monthly::all_months_in_a_year)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/monthly/2020/13")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}
