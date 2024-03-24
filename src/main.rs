use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use env_logger::Env;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
        .service(index)
        .service(hello)
        .wrap(Logger::default())
        .wrap(Logger::new("%a %t %r %s %b %{Referer}i %{User-Agent}i %T"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
