
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Email {
    pub smtp_host:String,// = "smtp.example.com"
    //smtp_host = "smtp.example.com"
    pub smtp_port: u32,// = 25
    //smtp_port = 25
    pub username:String, // = "master@domain.com"
   // username = "master@domain.com"
    pub password:String, // = "securitypassword"
   // password = "securitypassword"
    pub from_email:String, // = "example@example.com"
   // from_email = "example@example.com"
}