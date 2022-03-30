use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "h-captcha-response")]
    pub token: Option<String>,
}