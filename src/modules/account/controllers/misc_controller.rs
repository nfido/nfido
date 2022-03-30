use actix_session::Session;
use actix_web::{get, post, error, Error, HttpResponse, web, Responder, Result};
use crate::AppConfig;
use crate::common::vo::common_result::CommonResult;
use crate::model::nfido_members::NfidoMembers;

#[get("/account/sendVerifyEmail")]
pub async fn send_verify_email(session: Session,
                               conf: web::Data<AppConfig>) -> Result<impl Responder> {

    if session.get::<Option<i64>>("uid").is_err() {
      return  Ok(web::Json(CommonResult::<String> {
            code: 0,
            msg: "请先登录".to_string(),
            data: None,
        }))
    }

    //检测username
    //查数据库表，看昵称是不是被占用了
    let vu = rb
        .fetch_by_column::<Option<NfidoMembers>, _>("uid", session.get::<Option<i64>>("uid").unwrap())
        .await
        .unwrap();

    //没找到记录，登录失败
    if vu.is_none() {
        //查到 了记录
        return Ok(web::Json(CommonResult::<String> {
            code: 0,
            msg: "请先登录".to_string(),
            data: None,
        }));
    }


    //TODO 查用户资料，发邮件
    return Ok(web::Json(CommonResult::<String> {
        code: 0,
        msg: "己经发送".to_string(),
        data: None,
    }))

}

#[post("/account/emailVerify")]
pub async fn email_verify(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("account/login.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}
