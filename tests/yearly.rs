use actix_web::{test, web, App};
use rust_covid_api::{routes::yearly, types::YearlyCase};

#[cfg(test)]
mod all_years {
    use super::*;
    use chrono::Datelike;

    #[actix_web::test]
    async fn returns_all_years() {
        let current_year = chrono::Utc::now().year();
        let earliest_year = 2020;
        let app = test::init_service(
            App::new().service(web::scope("/yearly").service(yearly::all_years)),
        )
        .await;

        let req = test::TestRequest::with_uri("/yearly").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: Vec<YearlyCase> = test::read_body_json(resp).await;
        assert!(!body.is_empty());
        assert_eq!(body[0].year, earliest_year);
        assert_eq!(body.last().unwrap().year, current_year);
    }
}

#[cfg(test)]
mod specific_year {
    use super::*;

    #[actix_web::test]
    async fn returns_correct_data_given_valid_year() {
        let app = test::init_service(
            App::new().service(web::scope("/yearly").service(yearly::specific_year)),
        )
        .await;

        let chosen_year = 2020;
        let req = test::TestRequest::get()
            .uri(&format!("/yearly/{}", chosen_year))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let body: YearlyCase = test::read_body_json(resp).await;
        assert_eq!(body.year, chosen_year);
    }

    #[actix_web::test]
    async fn returns_404_given_invalid_year() {
        let app = test::init_service(
            App::new().service(web::scope("/yearly").service(yearly::specific_year)),
        )
        .await;

        let chosen_year = 2018;
        let req = test::TestRequest::get()
            .uri(&format!("/yearly/{}", chosen_year))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 404);
    }
}
