//! tests/health_check.rs
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Spin up an instance of our application
/// and return its address <br>
/// (i.e. http://localhost:xxxx)
// #[tokio::main]
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().expect("Failed to get local address");
    let address = format!("http://{port}");

    let mut config = get_configuration().expect("Failed to read configuration.");
    config.database.database_name = Uuid::new_v4().to_string();
    let con_pool = configure_database(&config.database).await;

    let server = zero2prod::startup::run(listener, con_pool.clone())
        .expect(format!("Failed to bind server to {port}").as_str());

    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: con_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let tp = spawn_app().await;
    println!("address -> {:?}", tp.address);
    let client = reqwest::Client::new();

    //act
    let _response = client
        .get(format!("{}/health_check", tp.address))
        .send()
        .await
        .expect("Failed to execute");
}

#[tokio::test]
async fn subscribe_returns_a_200() {
    //Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        // .header("Content-Type", "application/x-www-form-urlencoder")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute");

    //Assert
    let status = response.status().as_u16();
    assert_eq!(200, status);

    let saved = sqlx::query!("select email,name from subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");

    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both the name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
