use actix_web::{ web, App, HttpServer, dev::Service };
use std::sync::atomic::AtomicI32;
use futures_util::future::FutureExt;

mod state;
mod routes;

use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: "My Actix Web App".to_string(),
        counter: AtomicI32::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested1: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response1");
                    res
                })
            })
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested2: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response2");
                    res
                })
            })
            .service(routes::index)
            .configure(routes::app_config)
            .service(web::scope("/api").configure(routes::api_config))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
