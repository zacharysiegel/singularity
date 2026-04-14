use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::health::{DatabaseStatus, HealthResponse, HealthStatus};
use sqlx::PgPool;

use crate::http;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/health")
            .route(web::get().to(health_check)),
    );
}

async fn health_check(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let database_healthy = sqlx::query_scalar::<_, i32>("select 1")
        .fetch_one(pool.get_ref())
        .await
        .is_ok();

    let response = if database_healthy {
        HealthResponse {
            status: HealthStatus::Ok,
            database: DatabaseStatus::Connected,
        }
    } else {
        HealthResponse {
            status: HealthStatus::Degraded,
            database: DatabaseStatus::Unreachable,
        }
    };

    if database_healthy {
        http::serialize_response(&request, &response)
    } else {
        match serde_json::to_vec(&response) {
            Ok(bytes) => HttpResponse::ServiceUnavailable()
                .content_type("application/json")
                .body(bytes),
            Err(_) => HttpResponse::ServiceUnavailable().finish(),
        }
    }
}
