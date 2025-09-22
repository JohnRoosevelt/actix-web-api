use actix_web::middleware::Compress;

pub fn compression() -> Compress {
    Compress::default()
}
