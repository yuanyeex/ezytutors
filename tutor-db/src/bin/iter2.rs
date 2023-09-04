use crate::state::AppState;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Mutex;
use std::{env, io};

#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // construct app state
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'am good, you've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    // Construct app and configure routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(routes::general_routes)
            .configure(routes::course_routes)
    };
    // start http server
    HttpServer::new(app).bind("localhost:3000")?.run().await
}
