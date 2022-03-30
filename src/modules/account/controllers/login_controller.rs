use std::sync::Arc;
use actix_session::Session;
use actix_web::{get, post, error, web, Error, HttpResponse, Result};
use actix_web::web::Data;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rand::Rng;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use tera::{Context, Tera};
use crate::account::vo::login_form::LoginForm;
use crate::appconfig::appconfig::AppConfig;
use crate::captcha::hcaptcha_verify::hcaptch_verify;
use crate::model::nfido_members::NfidoMembers;

#[get("/account/login")]
pub async fn login(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("account/login.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}
#[post("/account/doLogin")]
pub async fn do_login(_in_req: web::Form<LoginForm>,
                   rb: web::Data<Arc<Rbatis>>,
                   session: Session,
                   tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{



    //模板的context
    let mut ctx = tera::Context::new();


    if _in_req.password.is_none() {

        ctx.insert("msg", "密码不能为空");
        return display_login_result(&tmpl, &conf, &ctx);
    }
    if _in_req.username.is_none() {

        ctx.insert("msg", "登录名不能为空");
        return display_login_result(&tmpl, &conf, &ctx);
    }

    //如果开启了captcha
    if conf.captcha_enabled == 1 {
        let input_token;
        match &_in_req.token {
            Some(token) => input_token = token,
            None => {
                log::info!(" token没有输入");
                let mut ctx = tera::Context::new();
                ctx.insert("msg", "验证未通过");
                return display_login_result(&tmpl, &conf, &ctx);
            }
        }
        let verify_result = hcaptch_verify(input_token.to_string(), conf.clone()).await;
        match verify_result {
            Ok(v) => {
                if v == true {
                    //TODO 验证通过
                    log::info!("验证通过: {}", v);
                }
            }
            Err(e) => {
                log::info!(" 验证失败, {}", e);

                ctx.insert("msg", "验证未通过");
                return display_login_result(&tmpl, &conf, &ctx);
            }
        }
    }



    //检测username
    //查数据库表，看昵称是不是被占用了
    let vu = rb
        .fetch_by_column::<Option<NfidoMembers>, _>("username", &_in_req.username)
        .await
        .unwrap();

    //没找到记录，登录失败
    if vu.is_none() {
        //查到 了记录
        log::info!(" 登录失败, {}", serde_json::to_string(&vu)?);


        ctx.insert("msg", "登录失败");
        return display_login_result(&tmpl, &conf, &ctx);
    }


    let v_profile = vu.unwrap();

    if v_profile.password.is_none() {

        ctx.insert("msg", "登录失败, 没有找到密码");
        return display_login_result(&tmpl, &conf, &ctx);
    }
    let pw = v_profile.password.unwrap();
    let hash = PasswordHash::new(&pw).unwrap();
    let result = Argon2::default().verify_password(_in_req.password.as_ref().unwrap().as_bytes(), &hash);
    match result {
        Ok(()) => {
            log::info!("ok ");
        },
        Err(e) => {
            log::info!(" error: {}", e);
            //登录失败，密码不对

            ctx.insert("msg", "登录失败，密码不对");
            return display_login_result(&tmpl, &conf, &ctx);
        }
    }


    //TODO 记录cookie等信息
    let v_uid = v_profile.uid.unwrap();
    log::info!("v_uid: {}", v_uid);
    session.insert("v_uid", v_uid);
    let v_username = v_profile.username.unwrap();
    log::info!("v_username: {}", v_username);
    session.insert("v_username", v_username);
    let v_verify_status = v_profile.verify_status.unwrap();
    log::info!("v_verify_status: {}", v_verify_status);
    session.insert("v_status", v_verify_status);


    let mut rng = rand::thread_rng();
    let verify_code = rng.gen_range(10000000..99999999);
    log::info!("生成的验证码为: {}", verify_code);
    session.insert("verify_code", verify_code);


    let vv_code = session.get::<i32>("verify_code");
    log::info!("vv_code: {}", vv_code.unwrap().unwrap());


    if v_verify_status != 1 {
        //需要验证邮箱了
        ctx.insert("msg", "点击进行邮箱验证");

        let s = tmpl.render("account/login_email_verify.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Termplate error"));

        log::info!("The site name: {}", conf.site_name.to_owned());
        return  Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
    }




    let s = tmpl.render("account/login_success.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}


fn display_login_result(tmpl: &Data<Tera>, conf: &Data<AppConfig>, x: &Context) -> std::result::Result<HttpResponse, Error> {
    let s = tmpl.render("account/reg_result.html", x)
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()));
}