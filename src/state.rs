
// This struct represents shared application state
pub struct AppState {
    pub app_name: String,
    pub counter: std::sync::Mutex<i32>,
}
