use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {:?}", &name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
