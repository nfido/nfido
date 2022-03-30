use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct EmailVerifyForm {

    pub verify_code: Option<i32>,
    #[serde(rename = "h-captcha-response")]
    pub token: Option<String>,
}