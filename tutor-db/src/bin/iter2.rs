#[path = "../iter2/models/mod.rs"]
mod models;

#[path = "../iter2/state.rs"]
mod state;

#[path = "../iter2/handlers/mod.rs"]
mod handlers;

#[path = "../iter2/routes.rs"]
mod routes;

#[path = "../iter2/dbaccess/db_access.rs"]
mod db_access;

#[path = "../iter2/errors.rs"]
mod errors;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::*;
use sqlx::PgPool;
use state::AppState;
use std::{env, io, sync::Mutex};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    HttpServer::new(app).bind(host_port)?.run().await
}
