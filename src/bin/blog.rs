use std::{env, io, net::TcpListener, sync::Mutex};
use actix_web::web;
use dotenv::dotenv;
use sqlx::PgPool;
use westfactor::{startup::run, state::AppState,};
use westfactor::telemetry::{get_subscriber, init_subscriber};


#[actix_rt::main]
pub async fn main() -> io::Result<()> {
    dotenv().ok();
    let subscriber = get_subscriber("west_blog".into(),"info".into(),std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;

    let application_task = tokio::spawn(application.run_until_stopped());

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