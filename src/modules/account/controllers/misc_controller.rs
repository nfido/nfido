use std::sync::Arc;
use actix_session::Session;
use actix_web::{get, post, error, Error, HttpResponse, web, Responder, Result};
use rand::Rng;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use crate::AppConfig;
use crate::common::vo::common_result::CommonResult;
use crate::mailer::mailer::MailDelivery;
use crate::mailer::mailer_smtp::MailerSmtp;
use crate::model::nfido_members::NfidoMembers;

#[get("/account/sendVerifyEmail")]
pub async fn send_verify_email(session: Session,
                               rb: web::Data<Arc<Rbatis>>,
                               conf: web::Data<AppConfig>) -> Result<impl Responder> {
    if session.get::<Option<i64>>("uid").is_err() {
        return Ok(web::Json(CommonResult::<String> {
            code: 0,
            msg: "请先登录".to_string(),
            data: None,
        }));
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


    let mailer = MailerSmtp {
        smtp_host: conf.email.smtp_host.to_string(),
        smtp_port: conf.email.smtp_port.to_string(),
        smtp_user: conf.email.username.to_string(),
        smtp_password: conf.email.password.to_string(),
        from_email: conf.email.from_email.to_string(),

    };


    let mut rng = rand::thread_rng();
    let verify_code = rng.gen_range(10000000..99999999);
    log::info!("生成的验证码为: {}", verify_code);
    session.insert("verify_code", verify_code);
    let sent_result = mailer.send_email(vu.unwrap().email.unwrap(), "您的邮箱验证码".to_string(), verify_code.to_string());
    match sent_result {
        Ok(_) => {
            log::info!(" 邮件发送成功");
        }
        Err(e) => {
            log::info!(" error: {}", e);
        }
    }
    //TODO 查用户资料，发邮件
    return Ok(web::Json(CommonResult::<String> {
        code: 0,
        msg: "己经发送".to_string(),
        data: None,
    }));
}

#[post("/account/emailVerify")]
pub async fn email_verify(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let s = tmpl.render("account/login.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}
