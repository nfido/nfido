use actix_web::{get, error, web, Error, HttpResponse, Result};
use crate::appconfig::appconfig::AppConfig;
#[get("/account/login")]
pub async fn login(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("account/login.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}

