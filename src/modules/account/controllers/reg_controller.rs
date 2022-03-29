use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{get, post, error, web, Error, HttpResponse, Result, Responder};
use actix_web::web::Data;
use rbatis::crud::{CRUD, Skip};
use rbatis::rbatis::Rbatis;
use rbatis::snowflake::new_snowflake_id;
use tera::{Context, Tera};
use crate::account::vo::check_email_req::CheckEmailReq;
use crate::account::vo::check_username_req::CheckUsernameReq;
use crate::account::vo::reg_form::RegForm;
use crate::appconfig::appconfig::AppConfig;
use crate::captcha::hcaptcha_verify::hcaptch_verify;
use crate::common::vo::common_result::CommonResult;
use crate::global_const::{ERROR_ACCOUNT_EMAIL_EXISTS, ERROR_ACCOUNT_USERNAME_EXISTS};
use crate::model::nfido_members::NfidoMembers;


#[get("/account/reg")]
pub async fn reg(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    //captcha key
    if conf.captcha_enabled == 1 {
        ctx.insert("captcha_key", &conf.h_captcha_site_key);
    }
    let s = tmpl
        .render("account/reg.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}

#[get("/account/check_username")]
pub async fn check_username(info: web::Query<CheckUsernameReq>, rb: web::Data<Arc<Rbatis>>) -> Result<impl Responder> {
//pub async fn check_username(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<impl Responder> {

    if info.username.eq("cnmade") {
        let err_result = CommonResult {
            code: ERROR_ACCOUNT_USERNAME_EXISTS,
            msg: "错误".to_string(),
            data: None,
        };
        return Ok(web::Json(err_result));
    }
    //查数据库表，看昵称是不是被占用了
    let vf = rb
        .fetch_by_column::<Option<NfidoMembers>, _>("username", &info.username)
        .await
        .unwrap();

    if vf.is_some() {
        //查到 了记录
        println!(" vf is {}", serde_json::to_string(&vf)?);
        let err_result = CommonResult {
            code: ERROR_ACCOUNT_EMAIL_EXISTS,
            msg: "用户名已经被注册".to_string(),
            data: None,
        };
        return Ok(web::Json(err_result));
    }
    println!("the vf: {}", serde_json::to_string(&vf)?);
    //log::info!(" The vf: " , serde_json::to_string(&vf)?);

    let mut data = HashMap::new();
    data.insert("test_key".to_string(), "value I like".to_string());
    let check_username_result = CommonResult {
        code: 0,
        msg: "可以注册".to_string(),
        data: Some(data),
    };
    Ok(web::Json(check_username_result))
}


#[get("/account/check_email")]
pub async fn check_email(info: web::Query<CheckEmailReq>, rb: web::Data<Arc<Rbatis>>) -> Result<impl Responder> {
//pub async fn check_username(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<impl Responder> {


    //查数据库表，看昵称是不是被占用了
    let vf = rb
        .fetch_by_column::<Option<NfidoMembers>, _>("email", &info.email)
        .await
        .unwrap();

    if vf.is_some() {
        //查到 了记录
        println!(" vf is {}", serde_json::to_string(&vf)?);
        let err_result = CommonResult {
            code: 45036,
            msg: "邮箱已经被注册".to_string(),
            data: None,
        };
        return Ok(web::Json(err_result));
    }
    println!("the vf: {}", serde_json::to_string(&vf)?);
    //log::info!(" The vf: " , serde_json::to_string(&vf)?);

    let mut data = HashMap::new();
    data.insert("test_key".to_string(), "value I like".to_string());
    let check_result = CommonResult {
        code: 0,
        msg: "可以注册".to_string(),
        data: Some(data),
    };
    Ok(web::Json(check_result))
}


#[post("/account/doReg")]
pub async fn do_reg(_in_req: web::Form<RegForm>,
                    rb: web::Data<Arc<Rbatis>>,
                    tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error> {


    //模板的context
    let mut ctx = tera::Context::new();
    //如果开启了captcha
    if conf.captcha_enabled == 1 {
        let input_token;
        match &_in_req.token {
            Some(token) => input_token = token,
            None => {
                log::info!(" token没有输入");
                let mut ctx = tera::Context::new();
                ctx.insert("msg", "验证未通过");
                return display_reg_result(&tmpl, &conf, &ctx);
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
                return display_reg_result(&tmpl, &conf, &ctx);
            }
        }
    }
    //检测username
    //查数据库表，看昵称是不是被占用了
    let vf_username = rb
        .fetch_by_column::<Option<NfidoMembers>, _>("username", &_in_req.username)
        .await
        .unwrap();

    if vf_username.is_some() {
        //查到 了记录
        log::info!(" 昵称被注册了, {}", serde_json::to_string(&vf_username)?);


        ctx.insert("msg", "昵称被注册了");
        return display_reg_result(&tmpl, &conf, &ctx);
    }
    //检测email
    //查数据库表，看昵称是不是被占用了
    let vf = rb
        .fetch_by_column::<Option<NfidoMembers>, _>("email", &_in_req.email)
        .await
        .unwrap();

    if vf.is_some() {
        //查到 了记录
        log::info!(" 邮件被注册了, {}", serde_json::to_string(&vf)?);

        ctx.insert("msg", "邮件被注册了");
        return display_reg_result(&tmpl, &conf, &ctx);
    }


    let mut uinfo = NfidoMembers::default();
    uinfo.uid = Option::from(new_snowflake_id());
    uinfo.username = _in_req.username.to_owned();
    uinfo.email = _in_req.email.to_owned();
    uinfo.password = _in_req.password.to_owned();


    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
           let i =  n.as_secs();
            println!("1970-01-01 00:00:00 UTC was {} seconds ago!", i);
            uinfo.regdate = Some(i);
        },
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    //TODO 处理注册逻辑
    let db_result = rb.save(&uinfo, &[Skip::Column("uid")]).await;
    match db_result {
        Ok(t) => {
            log::info!("db_result, last insert_id: {}", serde_json::to_string(&t)?);
            ctx.insert("msg", "注册成功");
        }
        Err(e) => {
            log::info!("error: {}", e);
            ctx.insert("msg", "注册失败");
        }
    };

    return display_reg_result(&tmpl, &conf, &ctx);
}

fn display_reg_result(tmpl: &Data<Tera>, conf: &Data<AppConfig>, x: &Context) -> std::result::Result<HttpResponse, Error> {
    let s = tmpl.render("account/reg_result.html", x)
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()));
}