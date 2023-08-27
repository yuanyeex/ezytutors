use super::models::Course;
use std::sync::Mutex;

pub struct AppState {
    // shared immutable state
    pub health_check_response: String,
    // shared mutable state
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
