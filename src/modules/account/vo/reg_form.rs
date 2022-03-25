use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct RegForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub token: String,
}