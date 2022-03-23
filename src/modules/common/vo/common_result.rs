use serde::{Serialize};

#[derive(Serialize,Deserialize)]
pub struct CommonResult<T> {
    //错误码，为0表示没有错误，大于0表示有错误
    pub code: u64,
    //消息提示
    pub msg: String,
    //具体的数据，是可选的，可以是Some，可以是None
    pub data: Option<T>,
}