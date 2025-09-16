use actix_web::{web, HttpResponse, Result};

#[derive(serde::Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

async fn get_products() -> Result<HttpResponse> {
    let products = vec![
        Product {
            id: 1,
            name: "Product A".to_string(),
            price: 29.99,
        },
        Product {
            id: 2,
            name: "Product B".to_string(),
            price: 49.99,
        },
    ];
    
    Ok(HttpResponse::Ok().json(products))
}

async fn get_product(path: web::Path<i32>) -> Result<HttpResponse> {
    let product_id = path.into_inner();
    let product = Product {
        id: product_id,
        name: format!("Product {}", product_id),
        price: (product_id * 10) as f64,
    };
    
    Ok(HttpResponse::Ok().json(product))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_products))
            .route("/{id}", web::get().to(get_product))
    );
}