use actix_web::{ web, HttpResponse, Result };
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthCheck {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

async fn health_check() -> Result<HttpResponse> {
    let health = HealthCheck {
        status: "OK".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Ok().json(health))
}

async fn readiness() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Ready"))
}

async fn liveness() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Alive"))
}

async fn info(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    data.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    println!("{data :?}");
    let new_val = data.counter.load(std::sync::atomic::Ordering::SeqCst);
    format!("hello {app_name}, request number {new_val}")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/health")
            .route("", web::get().to(health_check))
            .route("/ready", web::get().to(readiness))
            .route("/live", web::get().to(liveness))
            .route("/info", web::get().to(info))
    );
}
