use actix_web::{get, web, HttpResponse};

use crate::state::AppState;

// config the index route
#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Hello {app_name}! Request number: {counter}")
}

// config the /app
pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// config the /api/test
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
