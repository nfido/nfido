#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

pub mod modules;
use actix_web::{App, HttpServer};
use config::Config;

use modules::*;
use tera::Tera;
use std::borrow::Borrow;
use actix_files::Files;
use std::{env};
use std::sync::Arc;
use actix_web::web::Data;
use fast_log::config::Config as LogConfig;
use rbatis::rbatis::Rbatis;
use crate::appconfig::appconfig::AppConfig;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    //log
    fast_log::init(LogConfig::new().console()).unwrap();

    let listen_port_str = "127.0.0.1:8086";
    println!("Listening on http://{}", listen_port_str);


    //加载配置文件
    let settings = Config::builder()
        .add_source(config::File::with_name("./config.toml"))
        .build()
        .unwrap();

    //配置文件
    let app_config  = settings.try_deserialize::<AppConfig>().unwrap();




    //数据库加载


    //init rbatis . also you can use  pub static RB:Lazy<Rbatis> = Lazy::new(||Rbatis::new()); replace this
    log::info!("linking database...");
    let rb = Rbatis::new();
    rb.link(app_config.dbdsn.borrow()).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    log::info!("linking database successful!");




    HttpServer::new( move || {

        //处理模板
        let base_dir = env::current_dir().expect("not found path");


        log::info!("The base dir: {}", base_dir.to_str().expect(""));
        let template_dir_joined = base_dir.join("templates/**/*");
        let template_dir = template_dir_joined.to_str().expect("./templates/**/*");
        log::info!("The template dir: {}", template_dir.borrow());

        let tera =
            Tera::new(template_dir.borrow()).unwrap();


        //应用 开始
        App::new()
            .app_data(Data::new(tera.to_owned()))
            .app_data(Data::new(rb.to_owned()))
            // 注入配置
            .app_data(Data::new(app_config.to_owned()))
            .service(Files::new("/assets", "./public").index_file("index.html"))
            .service(home::controllers::home_controller::index)
            .service(home::controllers::about_controller::me)
            .service(home::controllers::admin_controller::login)
            .service(home::controllers::admin_controller::logout)
            .service(home::controllers::payment_controller::index)
            // 帖子列表页
            .service(forumdisplay::controllers::home_controller::index)
            // 帖子详情页
            .service(viewthread::controllers::home_controller::index)
            // 注册
            .service(account::controllers::reg_controller::reg)
            //登录
            .service(account::controllers::login_controller::login)
            //检测注册用户名
            .service(account::controllers::reg_controller::check_username)
            //检测注册邮箱
            .service(account::controllers::reg_controller::check_email)
    })
        .bind(listen_port_str)?
        .run()
        .await
}
