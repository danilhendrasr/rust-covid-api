use actix_web::HttpResponse;

#[derive(serde::Serialize)]
enum ServiceHealth {
    OK,
    _SourceUnavailable,
}

#[derive(serde::Serialize)]
struct Health {
    status: ServiceHealth,
}

// TODO: Provide more detailed informations
// E.g.
// 1. Source API status
// 2. Last data update
// 3. Uptime
// 4. Response time

pub async fn service_health() -> HttpResponse {
    let data = Health {
        status: ServiceHealth::OK,
    };

    HttpResponse::Ok().body(serde_json::to_string(&data).unwrap())
}
