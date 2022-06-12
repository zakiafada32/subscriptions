use sqlx::PgPool;
use std::net::TcpListener;
use subscription::configuration::Settings;
use subscription::startup::run;
use subscription::telemetry::{get_subscriber, init_subcriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = Settings::get_configuration().expect("Failed to load configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let subscriber = get_subscriber("subscription".into(), "info".into());
    init_subcriber(subscriber);

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, db_pool)?.await
}
