

use std::{env, io, net::TcpListener, sync::Mutex};
use actix_web::web;
use dotenv::dotenv;
use sqlx::PgPool;
use westfactor::{startup::run, state::AppState};


#[actix_rt::main]
pub async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host_address = env::var("HOST_ADDR").expect("host address");

    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let address = format!("{host_address:?}");

    let listener = TcpListener::bind(address)?;
    run(listener, shared_data)?.await?;
    Ok(())
    
}