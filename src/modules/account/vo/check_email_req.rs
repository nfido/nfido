use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CheckEmailReq {
    // 输入的email地址
    pub email: String,
}