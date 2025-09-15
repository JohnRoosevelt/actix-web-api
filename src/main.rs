use actix_web::{ web, App, HttpServer, dev::Service };
use std::sync::atomic::AtomicI32;
use futures_util::future::FutureExt;

mod state;
mod routes;
mod middleware;

use state::AppState;
use middleware::{ LoggerMiddleware, CorsMiddleware, AuthMiddleware };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    app_name: String::from("Actix Web"),
                    counter: AtomicI32::new(0),
                })
            )
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
            .wrap(LoggerMiddleware)
            .wrap(CorsMiddleware)
            .service(routes::index)
            .configure(routes::app_config)
            .service(web::scope("/api").wrap(AuthMiddleware).configure(routes::api_config))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
