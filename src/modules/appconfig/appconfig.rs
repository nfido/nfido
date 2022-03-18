

use crate::appconfig::object_storage;
use crate::appconfig::email;
use crate::appconfig::tongji_config;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    dbdsn: String,
    //  dbdsn="user=postgres password=postgres dbname=nfido host=127.0.0.1 port=5432 sslmode=disable TimeZone=Asia/Shanghai"
    admin_user: Vec<String>,
    // admin_user = ["admin","root"]
    admin_email:String,
    //admin_email = "admin@example.com"
    site_name:String,
    // site_name = "nfido"
    site_description:String,
    // "My memory about leobbs"
    //site_description = "My memory about leobbs"
    //# 加密密码用的盐
    key_of_encrypt:String,
    //= "nfidoabc1314"
    // key_of_encrypt = "nfidoabc1314"
    object_storage: object_storage::ObjectStorage,
    // [object_storage]
    email: email::Email,

    tongji_config: tongji_config::TongjiConfig,
}