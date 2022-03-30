
use actix_web::{error, Error, HttpResponse, web};
use crate::AppConfig;

#[get("/account/emailVerify")]
pub async fn login(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("account/login.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}
