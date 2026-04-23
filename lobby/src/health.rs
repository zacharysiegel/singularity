use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::health::{DatabaseStatusSerial, HealthResponseSerial, HealthStatusSerial};
use sqlx::PgPool;

use crate::http;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/health")
            .route(web::get().to(health_check)),
    );
}

async fn health_check(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let database_healthy: bool = sqlx::query_scalar::<_, i32>("select 1")
        .fetch_one(pool.get_ref())
        .await
        .is_ok();

    let (response, status_code): (HealthResponseSerial, StatusCode) = if database_healthy {
        (
            HealthResponseSerial {
                status: HealthStatusSerial::Ok,
                database: DatabaseStatusSerial::Connected,
            },
            StatusCode::OK,
        )
    } else {
        (
            HealthResponseSerial {
                status: HealthStatusSerial::Degraded,
                database: DatabaseStatusSerial::Unreachable,
            },
            StatusCode::SERVICE_UNAVAILABLE,
        )
    };

    http::serialize_response_with_status(&request, &response, status_code)
}
