

use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfidoMembers {
    pub uid: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    //uid               integer       default nextval('nfido_members_seq'::regclass) not null

   // username          varchar(32)   default ''::character varying                  not null

   // password          varchar(32)   default ''::character varying                  not null,
    pub regdate: Option<u64>,
    //regdate           integer       default 0                                      not null,
   pub postnum: Option<u64>,
   // postnum           bigint        default 0                                      not null,

   // email             varchar(60)   default ''::character varying                  not null,
   pub site: Option<String>,
   // site              varchar(75)   default ''::character varying                  not null,
    pub aim: Option<String>,
    //aim               varchar(40)   default ''::character varying                  not null,
   pub status: Option<String>,
   // status            varchar(35)   default ''::character varying                  not null,
   pub location: Option<String>,
   // location          varchar(50)   default ''::character varying                  not null,
   pub bio: Option<String>,
   // bio               text                                                         not null,
   pub sig: Option<String>,
   // sig               text                                                         not null,
   pub showemail: Option<String>,
   // showemail         varchar(15)   default ''::character varying                  not null,
   pub timeoffset: Option<i8>,
   // timeoffset        numeric(4, 2) default 0                                      not null,
   pub qq: Option<String>,
   // qq                varchar(30)   default ''::character varying                  not null,
  pub avatar: Option<String>,
  //  avatar            varchar(120)  default NULL::character varying,
  //  yahoo             varchar(40)   default ''::character varying                  not null,
   pub yahoo: Option<String>,
   // customstatus      varchar(250)  default ''::character varying                  not null,
   pub customstatus: Option<String>,
   // theme             smallint      default 0                                      not null,
   pub theme: Option<i8>,
   // bday              varchar(10)   default '0000-00-00'::character varying        not null,
  pub bday: Option<String>,
  //  langfile          varchar(40)   default ''::character varying                  not null,
  pub langfile: Option<String>,
  //  tpp               smallint      default 0                                      not null,
   pub tpp: Option<i8>,
   // ppp               smallint      default 0                                      not null,
   pub ppp: Option<i8>,
   // newsletter        char(3)       default ''::bpchar                             not null,
  pub newsletter: Option<String>,
  //  regip             varchar(15)   default ''::character varying                  not null,
   pub regip: Option<String>,
   // timeformat        integer       default 0                                      not null,
   pub timeformat: Option<i32>,
   // msn               varchar(40)   default ''::character varying                  not null,
   pub msn: Option<String>,
   // ban               varchar(15)   default '0'::character varying                 not null,
  pub ban: Option<String>,
  //  dateformat        varchar(10)   default ''::character varying                  not null,
  pub dateformat: Option<String>,
  //  ignoreu2u         text                                                         not null,
  pub ignoreu2u: Option<String>,
  //  lastvisit         integer       default 0                                      not null,
  pub lastvisit: Option<i32>,
  //  mood              varchar(128)  default 'Not Set'::character varying           not null,
  pub mood: Option<String>,
  //  pwdate            integer       default 0                                      not null,
  pub pwdate: Option<i32>,
  //  invisible         smallint      default 0,
  pub invisible: Option<i8>,
  //  u2ufolders        text                                                         not null,
    pub u2ufolders: Option<String>,
   // saveogu2u         char(3)       default ''::bpchar                             not null,
   pub saveogu2u: Option<String>,
   // emailonu2u        char(3)       default ''::bpchar                             not null,
   pub emailonu2u: Option<String>,
   // useoldu2u         char(3)       default ''::bpchar                             not null,
   pub useoldu2u: Option<String>,
   // u2ualert          smallint      default '0'::smallint                          not null,
   pub u2ualert: Option<String>,
   // bad_login_date    integer       default 0                                      not null,
   pub bad_login_date: Option<i64>,
   // bad_login_count   integer       default 0                                      not null,
  pub bad_login_count: Option<i64>,
  //  bad_session_date  integer       default 0                                      not null,
  pub bad_session_date: Option<i64>,
  //  bad_session_count integer       default 0                                      not null,
  pub bad_session_count: Option<i64>,
  //  sub_each_post     varchar(3)    default 'no'::character varying                not null,
  pub sub_each_post: Option<String>,
  //  waiting_for_mod   varchar(3)    default 'no'::character varying                not null
  pub waiting_for_mod: Option<String>,
    
}
 impl Default for NfidoMembers {

     fn default() -> Self {
         NfidoMembers{
             uid: None,
             username: None,
             email: None,
             password: None,
             regdate: None,
             postnum: None,
             site: None,
             aim: None,
             status: None,
             location: None,
             bio: Some("".parse().unwrap()),
             sig: Some("".parse().unwrap()),
             showemail: None,
             timeoffset: None,
             qq: None,
             avatar: None,
             yahoo: None,
             customstatus: None,
             theme: None,
             bday: None,
             langfile: None,
             tpp: None,
             ppp: None,
             newsletter: None,
             regip: None,
             timeformat: None,
             msn: None,
             ban: None,
             dateformat: None,
             ignoreu2u: Some("".parse().unwrap()),
             lastvisit: None,
             mood: None,
             pwdate: None,
             invisible: None,
             u2ufolders: Some("".parse().unwrap()),
             saveogu2u: None,
             emailonu2u: None,
             useoldu2u: None,
             u2ualert: None,
             bad_login_date: None,
             bad_login_count: None,
             bad_session_date: None,
             bad_session_count: None,
             sub_each_post: None,
             waiting_for_mod: None
         }
     }
 }