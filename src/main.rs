use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn call_awc() -> impl Responder {
    let client = awc::Client::default();
    let req = client.post("http://localhost:8080/echo");
    let mut res = req.send_body("abc").await.expect("Fail to send");
    let body = res.body().await.expect("Fail to get body");
    body
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(call_awc))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
