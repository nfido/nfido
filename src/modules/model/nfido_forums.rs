

use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfidoForums {
    pub uid: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub sig: Option<String>,
    pub ignoreu2u: Option<String>,
    pub u2ufolders: Option<String>,
    pub verify_status: Option<i8>,

}
impl Default for NfidoForums {

    fn default() -> Self {
        NfidoForums{
            uid: None,
            username: None,
            email: None,
            password: None,
            bio: Some("".parse().unwrap()),
            sig: Some("".parse().unwrap()),
            ignoreu2u: Some("".parse().unwrap()),
            u2ufolders: Some("".parse().unwrap()),
            verify_status: Some(0),
        }
    }
}