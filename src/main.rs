use std::net::TcpListener;

use actix_web::{get, post, web, App, HttpRequest, Responder};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    println!("{:?}", &listener);
    zero2prod::run(listener)?.await
}
