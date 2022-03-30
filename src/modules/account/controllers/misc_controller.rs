use std::sync::Arc;
use actix_session::Session;
use actix_web::{get, post, error, Error, HttpResponse, web, Responder, Result};
use actix_web::web::Data;

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use tera::{Context, Tera};
use crate::account::vo::email_verify_form::EmailVerifyForm;
use crate::AppConfig;
use crate::common::vo::common_result::CommonResult;
use crate::mailer::mailer::MailDelivery;
use crate::mailer::mailer_smtp::MailerSmtp;
use crate::model::nfido_members::NfidoMembers;

#[get("/account/sendVerifyEmail")]
pub async fn send_verify_email(session: Session,
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

#[post("/account/emailVerify")]
pub async fn email_verify(in_req: web::Form<EmailVerifyForm>,
                          session: Session,
                          rb: web::Data<Arc<Rbatis>>,
                          tmpl: web::Data<tera::Tera>,
                          conf: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let input_verify_code = in_req.verify_code.unwrap();

    let verify_code = session.get::<i32>("verify_code")?;
    log::info!("verify_code: {}", verify_code.unwrap());
    if session.get::<i64>("v_uid").is_err() {
        ctx.insert("msg", "请先登录");
        return display_misc_result(&tmpl, &conf, &ctx);
    }

    if verify_code.unwrap() != input_verify_code {
        session.remove("v_uid");
        session.remove("v_username");
        session.remove("v_verify_status");
        session.remove("verify_code");
        ctx.insert("msg", r#"请先 <a href="/account/login">登录</a>"#);
        return display_misc_result(&tmpl, &conf, &ctx);
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

        ctx.insert("msg", "请先登录");
        return display_misc_result(&tmpl, &conf, &ctx);
    }
    let mut n_profile = vu.unwrap();
    n_profile.verify_status = Some(1);


    let w = rb.new_wrapper().eq("uid", n_profile.uid);
    rb.update_by_wrapper(&n_profile, w, &[]).await;


    session.remove("v_verify_status");
    session.insert("v_verify_status", 1);

    let s = tmpl.render("account/login_success.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}


#[get("/account/logout")]
pub async fn logout(session: Session,
                    tmpl: web::Data<tera::Tera>,
                    conf: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    session.remove("v_uid");
    session.remove("v_username");
    session.remove("v_verify_status");
    session.remove("verify_code");
    ctx.insert("msg", r#"退出完毕，前往<a href="/">论坛首页</a>"#);
    return display_misc_result(&tmpl, &conf, &ctx);
}


fn display_misc_result(tmpl: &Data<Tera>, conf: &Data<AppConfig>, x: &Context) -> std::result::Result<HttpResponse, Error> {
    let s = tmpl.render("account/reg_result.html", x)
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()));
}