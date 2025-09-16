use actix_web::http::header;
use actix_cors::Cors;

pub fn configure() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().ends_with(b".example.com") ||
            origin.as_bytes().starts_with(b"http://localhost") ||
            origin.as_bytes().starts_with(b"http://127.0.0.1")
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .supports_credentials()
        .max_age(3600)
}