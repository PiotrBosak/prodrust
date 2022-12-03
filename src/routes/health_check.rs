use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, dev::Server};
use std::net::TcpListener;
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
