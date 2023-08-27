use std::sync::Mutex;
use super::models::Course;

pub struct AppState {
    // shared immutable state
    pub health_check_response: String,
    // shared mutable state
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
