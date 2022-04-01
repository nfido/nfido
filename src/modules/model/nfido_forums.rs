use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfidoForums {
    pub f_type: Option<i64>,
    pub fid: Option<i64>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub lastpost: Option<String>,
    pub moderator: Option<String>,
    pub displayorder: Option<i8>,
    pub description: Option<String>,
    pub allowsmilies: Option<String>,
    pub allowbbcode: Option<String>,
    pub userlist: Option<String>,
    pub theme: Option<i8>,
    pub posts: Option<i64>,
    pub threads: Option<i64>,
    pub fup: Option<i8>,
    pub postperm: Option<String>,
    pub allowimgcode: Option<String>,
    pub attachstatus: Option<String>,
    pub password: Option<String>,

    /* f_type         varchar(15)  default ''        not null,
     fid          smallint default nextval ('xmb_forums_seq')
     primary key,
     name         varchar(128) default ''        not null,
     status       varchar(15)  default ''        not null,
     lastpost     varchar(54)  default ''        not null,
     moderator    varchar(100) default ''        not null,
     displayorder smallint     default 0         not null,
     description  text                           null,
     allowsmilies char(3)      default ''        not null,
     allowbbcode  char(3)      default ''        not null,
     userlist     text                           not null,
     theme        smallint  default 0         not null,
     posts        int      default 0         not null,
     threads      int      default 0         not null,
     fup          smallint     default 0         not null,
     postperm     varchar(11)  default '0,0,0,0' not null,
     allowimgcode char(3)      default ''        not null,
     attachstatus varchar(15)  default ''        not null,
     password     varchar(32)  default ''        not null*/
}

impl Default for NfidoForums {
    fn default() -> Self {
        NfidoForums {
            f_type: None,
            fid: None,
            name: None,
            status: None,
            lastpost: None,
            moderator: None,
            displayorder: None,
            description: Some("".parse().unwrap()),
            allowsmilies: None,
            allowbbcode: None,
            userlist: None,
            theme: None,
            posts: None,
            threads: None,
            fup: None,
            postperm: None,
            allowimgcode: None,
            attachstatus: None,
            password: None
        }
    }
}