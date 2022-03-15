use actix_web::{get, Responder};

#[get("/payment")]
pub async fn index() -> impl Responder {
    format!(" want to payment")
}
