
use actix_web::{get, HttpResponse, Responder};

#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Operational")
}

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the API API!")
}