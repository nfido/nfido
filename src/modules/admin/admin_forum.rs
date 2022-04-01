use std::sync::Arc;
use actix_session::Session;
use actix_web::{Responder, web};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use tera::{Context, Tera};
use crate::account::vo::email_verify_form::EmailVerifyForm;
use crate::AppConfig;
use crate::common::vo::common_result::CommonResult;
use crate::mailer::mailer::MailDelivery;
use crate::mailer::mailer_smtp::MailerSmtp;
use crate::model::nfido_members::NfidoMembers;

#[get("/admin/forumList")]
pub async fn forum_list(session: Session,
                               rb: web::Data<Arc<Rbatis>>,
                               conf: web::Data<AppConfig>) -> Result<impl Responder> {
    let verify_code = session.get::<i32>("verify_code")?;
    log::info!("verify_code: {}", verify_code.unwrap());
    if session.get::<i64>("v_uid").is_err() {
        return Ok(web::Json(CommonResult::<String> {
            code: 0,
            msg: "请先登录".to_string(),
            data: None,
        }));
    }

    //检测username
    //查数据库表，看昵称是不是被占用了
    let uid = session.get::<i64>("v_uid")?.unwrap();
    log::info!("uid: {}", uid);
    let vu = rb
        .fetch_by_column::<Option<NfidoMembers>, _>("uid", uid)
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


    let verify_code = session.get::<i32>("verify_code").unwrap().unwrap();
    log::info!("verify_code: {}", verify_code);
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