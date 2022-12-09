use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
                insert into subscriptions (id, email, name, subscribed_at)
                values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
        )
        .execute(pool.as_ref())
        .await 
        { 
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("Failed");
                HttpResponse::InternalServerError().finish()
            }
            
        }
    
}
