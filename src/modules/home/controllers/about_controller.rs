use actix_web::{get, Responder};

#[get("/about")]
pub async fn me() -> impl Responder {
    format!(" nice to see you , i am bob smith")
}
