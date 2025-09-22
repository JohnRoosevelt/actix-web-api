use actix_web::{ web, App, HttpServer };
use std::sync::atomic::AtomicI32;

mod state;
mod routes;
mod middleware;

use state::AppState;
use middleware::{ init_logger, get_default_logger, configure_cors, compression };
use routes::configure_all_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        counter: AtomicI32::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(get_default_logger())
            .wrap(configure_cors())
            .wrap(compression())
            .configure(configure_all_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
