use actix_web::{get, Responder};

#[get("/admin/login")]
pub async fn login() -> impl Responder {
    format!("welcome admin login")
}
#[get("/admin/logout")]
pub async fn logout() -> impl Responder {
    format!(" bye bye , my admin")
}
