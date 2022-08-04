use actix_web::HttpResponse;
use utoipa::Component;

#[derive(serde::Serialize, Component)]
pub enum ServiceStatus {
    OK,
    _SourceUnavailable,
}

#[derive(serde::Serialize, Component)]
pub struct ServiceHealth {
    status: ServiceStatus,
}

// TODO: Provide more detailed informations
// E.g.
// 1. Source API status
// 2. Last data update
// 3. Uptime
// 4. Response time

/// Inspect service's health.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Monitoring",
    responses(
        (status = 200, description = "Success processing daily cases summary.", body = ServiceHealth),
        (status = 500, description = "Something went wrong during the processing.", body = String),
    )
)]
pub async fn service_health() -> HttpResponse {
    let data = ServiceHealth {
        status: ServiceStatus::OK,
    };

    HttpResponse::Ok().json(data)
}
