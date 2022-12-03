use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, dev::Server};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

