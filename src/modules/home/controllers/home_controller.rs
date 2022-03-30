use actix_session::Session;
use actix_web::{get, error, web, Error, HttpResponse, Result};
use crate::appconfig::appconfig::AppConfig;

#[get("/")]
pub async fn index(session: Session, tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{

    let mut ctx = tera::Context::new();
    if session.get::<i64>("v_uid").is_ok() {
        let uid = session.get::<i64>("v_uid")?;
        if uid.is_some() {
            ctx.insert("v_uid", &uid.unwrap());
            ctx.insert("v_username", &session.get::<String>("v_username")?.unwrap());
        }
    }
    let s = tmpl.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}
