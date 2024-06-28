use std::{net::TcpListener, sync::Mutex};

use actix_web::{dev::Server, web};
use sqlx::{postgres::PgPoolOptions, PgPool};
use westfactor::{
    configuration::{DatabaseSettings, Settings},
    startup::run,
    state::AppState,
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let shared_data = web::Data::new(AppState {
            health_check_response: "I'm good. You've already asked me ".to_string(),
            visit_count: Mutex::new(0),
            db: connection_pool,
        });
        let server = run(listener, shared_data).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);
