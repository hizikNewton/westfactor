#[path = "../blog/mod.rs"]
mod blog;

use blog::blog_App;
use dotenv::dotenv;
use sqlx::PgPool;
use std::net::TcpListener;
use westfactor::configuration::get_configuration;
use westfactor::startup::run;
use westfactor::telemetry::{get_subscriber, init_subscriber};

#[actix_rt::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let subscriber = get_subscriber("west_blog".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = blog_App::Application::build(configuration.clone()).await?;

    let application_task = tokio::spawn(application.run_until_stopped());

    Ok(())
}
