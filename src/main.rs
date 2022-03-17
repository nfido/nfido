
pub mod modules;
use actix_web::{ App, HttpServer};

use modules::*;
use tera::Tera;
use std::borrow::Borrow;
use actix_files::Files;
use std::{env};
use actix_web::web::Data;


extern crate log;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    let listen_port_str = "127.0.0.1:8086";
    println!("Listening on http://{}", listen_port_str);

    HttpServer::new(|| {

        let base_dir = env::current_dir().expect("not found path");


        println!("The base dir: {}", base_dir.to_str().expect(""));
        let template_dir_joined = base_dir.join("templates/**/*");
        let template_dir = template_dir_joined.to_str().expect("./templates/**/*");
        println!("The template dir: {}", template_dir.borrow());

        let tera =
            Tera::new(template_dir.borrow()).unwrap();
        App::new()
            .app_data(Data::new(tera))
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
    })
        .bind(listen_port_str)?
        .run()
        .await
}
