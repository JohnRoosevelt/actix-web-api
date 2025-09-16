pub mod user;
pub mod product;
pub mod auth;
pub mod health;

pub use user::configure as configure_user_routes;
pub use product::configure as configure_product_routes;
pub use auth::configure as configure_auth_routes;
pub use health::configure as configure_health_routes;

pub fn configure_all_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .configure(configure_health_routes)
            .configure(configure_auth_routes)
            .configure(configure_user_routes)
            .configure(configure_product_routes)
    );
}