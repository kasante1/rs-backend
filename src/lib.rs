use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}


#[get("/health-check")]
async fn health_check()-> impl Responder {
    HttpResponse::Ok()
}


pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .service(index)
        .service(hello)
        .service(health_check)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}