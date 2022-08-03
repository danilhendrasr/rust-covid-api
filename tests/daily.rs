use actix_web::{test, web, App};
use rust_covid_api::{routes::daily, types::DailyItem};

mod all_days {
    use actix_web_lab::middleware::from_fn;
    use chrono::Datelike;

    use super::*;

    #[actix_web::test]
    async fn returns_all_days() {
        let current_time = chrono::Utc::now();
        let current_year = current_time.year();
        let current_month = current_time.month();
        let current_day = current_time.day();

        let earliest_year = 2020;
        let earliest_month = 3;
        let earliest_day = 2;

        let app = test::init_service(
            App::new().service(
                web::scope("/daily")
                    .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                    .service(daily::index_handler),
            ),
        )
        .await;

        let req = test::TestRequest::get().uri("/daily").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: Vec<DailyItem> = test::read_body_json(resp).await;
        assert!(!body.is_empty());
        assert_eq!(body[0].year, earliest_year);
        assert_eq!(body[0].month, earliest_month);
        assert_eq!(body[0].day, earliest_day);

        let last_item = body.last().unwrap();
        assert_eq!(last_item.year, current_year);
        assert_eq!(last_item.month, current_month);

        // There's a chance the last item in the response is yesterday's daily case
        // because the API hasn't updated its data.
        if current_day - last_item.day >= 2 {
            panic!();
        }
    }
}

mod all_days_in_a_year {
    use actix_web_lab::middleware::from_fn;

    use super::*;

    #[actix_web::test]
    async fn returns_all_days_in_a_year() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                .service(web::scope("/daily").service(daily::specific_year)),
        )
        .await;

        let chosen_year = 2020;
        let req = test::TestRequest::get()
            .uri(&format!("/daily/{chosen_year}"))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: Vec<DailyItem> = test::read_body_json(resp).await;
        assert_eq!(body[0].year, chosen_year);
        assert_eq!(body.last().unwrap().year, chosen_year);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_year() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                .service(web::scope("/daily").service(daily::specific_year)),
        )
        .await;

        let req = test::TestRequest::get().uri("/daily/2018").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}

mod all_days_in_a_month {
    use actix_web_lab::middleware::from_fn;

    use super::*;

    #[actix_web::test]
    async fn returns_all_days_in_a_month() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                .service(web::scope("/daily").service(daily::specific_month)),
        )
        .await;

        let chosen_year = 2021;
        let chosen_month = 3;
        let req = test::TestRequest::get()
            .uri(&format!("/daily/{chosen_year}/{chosen_month}"))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: Vec<DailyItem> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 31);
        assert_eq!(body[0].year, chosen_year);
        assert_eq!(body[0].month, chosen_month);

        let last_item = body.last().unwrap();
        assert_eq!(last_item.year, chosen_year);
        assert_eq!(last_item.month, chosen_month);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_month() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                .service(web::scope("/daily").service(daily::specific_month)),
        )
        .await;

        let req = test::TestRequest::get().uri("/daily/2021/0").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}

mod specific_day {
    use actix_web_lab::middleware::from_fn;

    use super::*;

    #[actix_web::test]
    async fn returns_200_given_valid_day() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                .service(web::scope("/daily").service(daily::specific_day)),
        )
        .await;

        let chosen_year = 2021;
        let chosen_month = 2;
        let chosen_day = 26;
        let req = test::TestRequest::get()
            .uri(&format!("/daily/{chosen_year}/{chosen_month}/{chosen_day}"))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: DailyItem = test::read_body_json(resp).await;
        assert_eq!(body.year, chosen_year);
        assert_eq!(body.month, chosen_month);
        assert_eq!(body.day, chosen_day);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_day() {
        let app = test::init_service(
            App::new()
                .wrap(from_fn(daily::middleware::filter_malformed_query_params))
                .service(web::scope("/daily").service(daily::specific_day)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/daily/2021/2/31")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}
