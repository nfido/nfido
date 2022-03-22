use std::collections::HashMap;
use actix_web::{get, error, web, Error, HttpResponse, Result, Responder};
use crate::appconfig::appconfig::AppConfig;
use crate::account::vo::check_username_result::CheckUsernameResult;


#[get("/account/reg")]
pub async fn reg(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("account/reg.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}

#[get("/account/check_username")]
pub async fn check_username() -> Result<impl Responder> {
//pub async fn check_username(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<impl Responder> {

    let mut data = HashMap::new();
    data.insert("test_key".to_string(), "value I like".to_string());
    let check_username_result = CheckUsernameResult{
        code: 0,
        msg: "".to_string(),
        data,
    };
    Ok(web::Json(check_username_result))
}

