use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
