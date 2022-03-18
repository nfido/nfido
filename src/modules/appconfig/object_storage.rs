use serde::{Serialize, Deserialize};
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectStorage {

    aws_access_key_id:String, //= "*****"
    //aws_access_key_id = "*****"
    aws_secret_access_key:String, // = "******"
    //aws_secret_access_key = "******"
    aws_region:String, // = "us-east-1"
    //aws_region = "us-east-1"
    aws_bucket:String, // = "www.domain.com"
    //aws_bucket = "www.domain.com"
    cdn_url: String, // = "//www.domain.com"
    //cdn_url = "//www.domain.com"
}