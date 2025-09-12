
// This struct represents shared application state
#[derive(Debug)]
pub struct AppState {
    pub app_name: String,
    // pub counter: std::sync::Mutex<i32>,  // sync lock for safe concurrent access
    // pub counter: tokio::sync::Mutex<i32>, // async lock for safe concurrent access
    pub counter: std::sync::atomic::AtomicI32, // atomic for safe concurrent access
}
