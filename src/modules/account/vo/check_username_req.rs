use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CheckUsernameReq {
    // 输入的用户名
    pub username: String,
}