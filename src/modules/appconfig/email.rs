
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Email {
    smtp_host:String,// = "smtp.example.com"
    //smtp_host = "smtp.example.com"
    smtp_port: u32,// = 25
    //smtp_port = 25
    username:String, // = "master@domain.com"
   // username = "master@domain.com"
    password:String, // = "securitypassword"
   // password = "securitypassword"
    from_email:String, // = "example@example.com"
   // from_email = "example@example.com"
}