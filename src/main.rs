use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //Panic if we can't read config
    let config = get_configuration().expect("Failed to read configuration.");
    let con_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)
        .expect(format!("Failed to bind to port{}", config.application_port).as_str());
    run(listener, con_pool)?.await
}
