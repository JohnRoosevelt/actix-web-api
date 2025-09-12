use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::{
    collections::HashMap,
    sync::atomic::Ordering,
    task::{Context, Poll},
    rc::Rc,
    cell::RefCell,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RequestStatsMiddleware {
    global_counter: Arc<std::sync::atomic::AtomicI32>,
    endpoint_counter: Arc<Mutex<HashMap<String, i32>>>,
}

impl RequestStatsMiddleware {
    pub fn new() -> Self {
        Self {
            global_counter: Arc::new(std::sync::atomic::AtomicI32::new(0)),
            endpoint_counter: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_stats(&self) -> (i32, HashMap<String, i32>) {
        let global_count = self.global_counter.load(Ordering::SeqCst);
        let endpoint_count = self.endpoint_counter.lock().await.clone();
        (global_count, endpoint_count)
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestStatsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestStatsService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestStatsService {
            service: Rc::new(RefCell::new(service)),
            global_counter: self.global_counter.clone(),
            endpoint_counter: self.endpoint_counter.clone(),
        }))
    }
}

pub struct RequestStatsService<S> {
    service: Rc<RefCell<S>>,
    global_counter: Arc<std::sync::atomic::AtomicI32>,
    endpoint_counter: Arc<Mutex<HashMap<String, i32>>>,
}

impl<S, B> Service<ServiceRequest> for RequestStatsService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.borrow_mut().poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let global_counter = self.global_counter.clone();
        let endpoint_counter = self.endpoint_counter.clone();
        
        let path = req.path().to_string();
        
        Box::pin(async move {
            global_counter.fetch_add(1, Ordering::SeqCst);
            
            let mut endpoint_map = endpoint_counter.lock().await;
            *endpoint_map.entry(path.clone()).or_insert(0) += 1;
            
            drop(endpoint_map);
            
            req.extensions_mut()
                .insert(global_counter.clone());
            
            let fut = service.borrow_mut().call(req);
            fut.await
        })
    }
}