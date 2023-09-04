use std::{env, io};
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use crate::routes::{course_routes, general_routes};
use crate::state::AppState;

#[path = "../iter3/handlers.rs"]
mod handlers;

#[path = "../iter3/models.rs"]
mod models;

#[path = "../iter3/routes.rs"]
mod routes;

#[path = "../iter3/state.rs"]
mod state;

#[path = "../iter3/db_access.rs"]
mod db_access;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState{
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    // Start the http server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
