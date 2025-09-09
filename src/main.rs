use actix_web::{ web, App, HttpServer };
use std::sync::Mutex;

mod state;
mod routes;

use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: "My Actix Web App".to_string(),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(routes::index)
            .configure(routes::app_config)
            .service(web::scope("/api").configure(routes::api_config))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
