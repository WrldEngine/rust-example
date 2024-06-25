use actix_web::{get, post, HttpResponse, Responder};
use log::info;


#[get("/")]
async fn hello() -> impl Responder {
    info!("hello handler started");
    HttpResponse::Ok().body("Hello world!")
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    info!("echo handler started");
    HttpResponse::Ok().body(req_body)
}