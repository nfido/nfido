use serde::{Serialize, Deserialize};
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct TongjiConfig {

    tongji_enabled: u8,
    //tongji_enabled=1
    tongji_code:String,
    //tongji_code='''

}