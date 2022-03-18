
use actix_web::{get, error, web, Error, HttpResponse, Result};
use crate::AppConfig;

#[get("/")]
pub async fn index(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    println!("The config: {}", serde_json::to_string(conf.get_ref()).unwrap());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}
