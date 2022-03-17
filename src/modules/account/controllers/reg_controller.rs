use actix_web::{get, Responder};

#[get("/account/reg")]
pub async fn reg() -> impl Responder {
    format!(" nice to see you , i am bob smith")
}
