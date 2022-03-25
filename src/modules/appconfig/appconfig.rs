use crate::appconfig::object_storage;
use crate::appconfig::email;
use crate::appconfig::tongji_config;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub dbdsn: String,
    //  dbdsn="user=postgres password=postgres dbname=nfido host=127.0.0.1 port=5432 sslmode=disable TimeZone=Asia/Shanghai"
    pub admin_user: Vec<String>,
    // admin_user = ["admin","root"]
    pub admin_email: String,
    //admin_email = "admin@example.com"
    pub site_name: String,
    // site_name = "nfido"
    pub site_description: String,
    // "My memory about leobbs"
    //site_description = "My memory about leobbs"
    //# 加密密码用的盐
    pub key_of_encrypt: String,
    //= "nfidoabc1314"
    // key_of_encrypt = "nfidoabc1314"


    //是否启用 hcaptcha
    pub captcha_enabled: i8,
    //=0
    // hcaptcha site key
    pub h_captcha_site_key: String,
    //= ""
    // hcaptch 密匙
    pub h_captcha_secret_key: String,
    //= ""
    pub object_storage: object_storage::ObjectStorage,
    // [object_storage]
    pub email: email::Email,

    pub tongji_config: tongji_config::TongjiConfig,
}