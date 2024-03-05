//! tests/health_check.rs
use std::net::TcpListener;

/// Spin up an instance of our application
/// and return its address <br>
/// (i.e. http://localhost:xxxx)
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let sock_addr = listener.local_addr().expect("Failed to get local address");

    let server =
        zero2prod::run(listener).expect(format!("Failed to bind server to {sock_addr}").as_str());

    let _ = tokio::spawn(server);
    format!("http://{sock_addr}")
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    println!("address -> {:?}", address);
    let client = reqwest::Client::new();

    //act
    let _response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute");
}

use std::sync::Arc;
#[tokio::test]
async fn subscribe_returns_a_200() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &address))
        // .header("Content-Type", "application/x-www-form-urlencoder")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute");

    //Assert
    let status = response.status().as_u16();
    // println!("text {:?}", response.);
    assert_eq!(200, status);
}

#[tokio::test]
async fn subscribe_returns_a_400() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both the name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(format!("{}/subscriptions", &address))
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
