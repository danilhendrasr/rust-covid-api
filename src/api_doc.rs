use crate::{
    routes::{
        daily,
        health::{self, ServiceHealth, ServiceStatus},
        index::{self, CasesSummary},
        monthly, yearly,
    },
    types::{DailyCase, MonthlyCase, YearlyCase},
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    handlers(
        index::daily_cases_summary,
        health::service_health,
        yearly::all_years,
        yearly::specific_year,
        monthly::all_months,
        monthly::all_months_in_a_year,
        monthly::specific_month,
        daily::all_days,
        daily::all_days_in_a_month,
        daily::all_days_in_a_year,
        daily::specific_day,
    ),
    components(
        CasesSummary,
        ServiceHealth,
        ServiceStatus,
        YearlyCase,
        MonthlyCase,
        DailyCase
    )
)]
pub struct ApiDoc;

pub enum EndpointTags {
    DataEndpoints,
    MonitoringEndpoints,
}
