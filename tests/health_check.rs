#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    //act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute");
}

fn spawn_app() {
    let server = zero2prod::run(None).expect("Failed to bind server on port 8000");

    let _ = tokio::spawn(server);
}
