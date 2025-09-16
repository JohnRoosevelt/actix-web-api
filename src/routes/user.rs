use actix_web::{web, HttpResponse, Result};
use crate::middleware::AuthMiddleware;

#[derive(serde::Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

async fn get_users() -> Result<HttpResponse> {
    let users = vec![
        UserResponse {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
        },
        UserResponse {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
        },
    ];
    
    Ok(HttpResponse::Ok().json(users))
}


async fn get_user(path: web::Path<i32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let user = UserResponse {
        id: user_id,
        username: format!("user{}", user_id),
        email: format!("user{}@example.com", user_id),
    };
    
    Ok(HttpResponse::Ok().json(user))
}

async fn create_user(user: web::Json<CreateUser>) -> Result<HttpResponse> {
    let new_user = UserResponse {
        id: 100,
        username: user.username.clone(),
        email: user.email.clone(),
    };
    
    Ok(HttpResponse::Created().json(new_user))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(AuthMiddleware)
            .route("", web::get().to(get_users))
            .route("/{id}", web::get().to(get_user))
            .route("", web::post().to(create_user))
    );
}