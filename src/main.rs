use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    zero2prod::run(None)?.await
}
