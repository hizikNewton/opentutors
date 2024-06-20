#[path ="../iter2/models.rs"]
mod models;

#[path ="../iter2/state.rs"]
mod state;

#[path ="../iter2/handlers.rs"]
mod handlers;

#[path ="../iter2/routers.rs"]
mod routers;

#[path ="../iter2/db_access.rs"]
mod db_access;

#[path ="../iter2/errors.rs"]
mod errors;

use std::{env, io, sync::Mutex};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use state::AppState;
use routers::*;

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
    };
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    HttpServer::new(app).bind(host_port)?.run().await
}