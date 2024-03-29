use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration,
    email_client::{self, EmailClient},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //Panic if we can't read config
    let config = get_configuration().expect("Failed to read configuration.");

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address.clone())
        .expect(format!("Failed to bind to address{}", address).as_str());

    let con_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    //sender avoids moving because of cloning?
    let email_client = EmailClient::new(
        config.email_client.base_url.clone(),
        config
            .email_client
            .sender()
            .expect("Invalid Sender Email Address"),
        config.email_client.authorization_token,
    );

    run(listener, con_pool, email_client)?.await
}
