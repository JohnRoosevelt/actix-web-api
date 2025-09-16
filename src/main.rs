use actix_web::{ web, App, HttpServer };
use std::sync::atomic::AtomicI32;

mod state;
mod routes;
mod middleware;

use state::AppState;
use middleware::{ LoggerMiddleware, configure_cors };
use routes::configure_all_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        counter: AtomicI32::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(LoggerMiddleware)
            .wrap(configure_cors())
            .configure(configure_all_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
