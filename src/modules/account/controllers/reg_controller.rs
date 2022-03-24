use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{get, error, web, Error, HttpResponse, Result, Responder};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use crate::account::vo::check_username_req::CheckUsernameReq;
use crate::appconfig::appconfig::AppConfig;
use crate::common::vo::common_result::CommonResult;
use crate::model::nfido_members::NfidoMembers;


#[get("/account/reg")]
pub async fn reg(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("account/reg.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}

#[get("/account/check_username")]
pub async fn check_username(info: web::Query<CheckUsernameReq>, rb: web::Data<Arc<Rbatis>>) -> Result<impl Responder> {
//pub async fn check_username(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<impl Responder> {

    if info.username.eq("cnmade") {
        let err_result = CommonResult {
            code: 45036,
            msg: "错误".to_string(),
            data: None
        };
        return Ok(web::Json(err_result));

    }
    //查数据库表，看昵称是不是被占用了
    let vf = rb.fetch_by_column::<Option<NfidoMembers>, _>("username", info.username.to_string()).await.unwrap();

    if vf.is_some() {
        //查到 了记录
        println!(" vf is {}", serde_json::to_string(&vf)?);
        let err_result = CommonResult {
            code: 45036,
            msg: "用户名已经被注册".to_string(),
            data: None
        };
        return Ok(web::Json(err_result));
    }
    println!("the vf: {}", serde_json::to_string(&vf)?);
    //log::info!(" The vf: " , serde_json::to_string(&vf)?);

    let mut data= HashMap::new();
    data.insert("test_key".to_string(), "value I like".to_string());
    let check_username_result = CommonResult{
        code: 0,
        msg: "".to_string(),
        data: Some(data),
    };
    Ok(web::Json(check_username_result))
}

