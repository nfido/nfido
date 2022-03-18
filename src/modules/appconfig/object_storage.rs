use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectStorage {

    pub aws_access_key_id:String, //= "*****"
    //aws_access_key_id = "*****"
    pub aws_secret_access_key:String, // = "******"
    //aws_secret_access_key = "******"
    pub aws_region:String, // = "us-east-1"
    //aws_region = "us-east-1"
    pub aws_bucket:String, // = "www.domain.com"
    //aws_bucket = "www.domain.com"
    pub cdn_url: String, // = "//www.domain.com"
    //cdn_url = "//www.domain.com"
}