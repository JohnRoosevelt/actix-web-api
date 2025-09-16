use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::middleware::AuthMiddleware;
use serde_json;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: i64,
    pub message: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

async fn login(credentials: web::Json<LoginRequest>) -> Result<HttpResponse> {
    if credentials.username == "admin" && credentials.password == "password" {
        let response = LoginResponse {
            token: "fake-jwt-token-12345".to_string(),
            expires_in: 3600,
            message: "Login successful".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::Unauthorized().json(
            serde_json::json!({"error": "Invalid credentials"})
        ))
    }
}

async fn logout() -> Result<HttpResponse> {
    let response = LogoutResponse {
        message: "Logged out successfully".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login)) 
            .route("/logout", web::post().to(logout).wrap(AuthMiddleware))
    );
}