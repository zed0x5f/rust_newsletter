use secrecy::ExposeSecret;
// use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //Panic if we can't read config
    let config = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)
        .expect(format!("Failed to bind to port{}", config.application_port).as_str());

    let con_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    run(listener, con_pool)?.await
}
