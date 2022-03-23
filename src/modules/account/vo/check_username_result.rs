use serde::{Serialize};

#[derive(Serialize)]
pub struct CheckUsernameResult<T> {
    pub code: u64,
    pub msg: String,
    pub data: Option<T>,
}