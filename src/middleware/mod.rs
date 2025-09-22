mod auth;
mod cors;
mod compression;
mod envlogger;

pub use auth::AuthMiddleware;
pub use cors::configure as configure_cors;
pub use compression::compression;
pub use envlogger::{init_logger, get_default_logger};