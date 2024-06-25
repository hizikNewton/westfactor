use actix_web::web;
use sqlx::PgPool;
use std::{net::TcpListener, sync::Mutex};
use westfactor::{configuration::get_configuration, startup::run, state::AppState};

struct TestApp {
    address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = "testdb".into();
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: connection_pool.clone(),
    });

    let server = run(listener, shared_data).expect("Failed to bind address");
    let _ = actix_rt::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
