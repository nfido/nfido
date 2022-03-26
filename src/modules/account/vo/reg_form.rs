use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct RegForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "h-captcha-response")]
    pub token: Option<String>,
}