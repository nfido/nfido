
use actix_web::{get, error, web, Error, HttpResponse, Result};


#[get("/")]
pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}
