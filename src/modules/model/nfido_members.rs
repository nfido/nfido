

use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfidoMembers {
    pub uid: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    
}
 impl Default for NfidoMembers {

     fn default() -> Self {
         NfidoMembers{
             uid: None,
             username: None,
             email: None,
         }
     }
 }