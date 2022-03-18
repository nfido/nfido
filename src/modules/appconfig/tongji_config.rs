use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct TongjiConfig {

    pub tongji_enabled: u8,
    //tongji_enabled=1
    pub tongji_code:String,
    //tongji_code='''

}