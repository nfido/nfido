-- nfido_members 开始

-- auto-generated definition
create sequence nfido_members_seq;

alter sequence nfido_members_seq owner to postgres;

-- auto-generated definition
create table nfido_members
(
    uid               integer       default nextval('nfido_members_seq'::regclass) not null
        primary key,
    username          varchar(32)   default ''::character varying                  not null
        constraint userunique
            unique,
    password          varchar(256)  default ''::character varying                  not null,
    regdate           integer       default 0                                      not null,
    postnum           bigint        default 0                                      not null,
    email             varchar(60)   default ''::character varying                  not null,
    site              varchar(75)   default ''::character varying                  not null,
    aim               varchar(40)   default ''::character varying                  not null,
    status            varchar(35)   default ''::character varying                  not null,
    location          varchar(50)   default ''::character varying                  not null,
    bio               text                                                         not null,
    sig               text                                                         not null,
    showemail         varchar(15)   default ''::character varying                  not null,
    timeoffset        numeric(4, 2) default 0                                      not null,
    qq                varchar(30)   default ''::character varying                  not null,
    avatar            varchar(120)  default NULL::character varying,
    yahoo             varchar(40)   default ''::character varying                  not null,
    customstatus      varchar(250)  default ''::character varying                  not null,
    theme             smallint      default 0                                      not null,
    bday              varchar(10)   default '0000-00-00'::character varying        not null,
    langfile          varchar(40)   default ''::character varying                  not null,
    tpp               smallint      default 0                                      not null,
    ppp               smallint      default 0                                      not null,
    newsletter        char(3)       default ''::bpchar                             not null,
    regip             varchar(15)   default ''::character varying                  not null,
    timeformat        integer       default 0                                      not null,
    msn               varchar(40)   default ''::character varying                  not null,
    ban               varchar(15)   default '0'::character varying                 not null,
    dateformat        varchar(10)   default ''::character varying                  not null,
    ignoreu2u         text                                                         not null,
    lastvisit         integer       default 0                                      not null,
    mood              varchar(128)  default 'Not Set'::character varying           not null,
    pwdate            integer       default 0                                      not null,
    invisible         smallint      default 0,
    u2ufolders        text                                                         not null,
    saveogu2u         char(3)       default ''::bpchar                             not null,
    emailonu2u        char(3)       default ''::bpchar                             not null,
    useoldu2u         char(3)       default ''::bpchar                             not null,
    u2ualert          smallint      default '0'::smallint                          not null,
    bad_login_date    integer       default 0                                      not null,
    bad_login_count   integer       default 0                                      not null,
    bad_session_date  integer       default 0                                      not null,
    bad_session_count integer       default 0                                      not null,
    sub_each_post     varchar(3)    default 'no'::character varying                not null,
    waiting_for_mod   varchar(3)    default 'no'::character varying                not null,
    verify_status     smallint      default 0
);

alter table nfido_members
    owner to postgres;

create index lastvisit
    on nfido_members (lastvisit);

-- nfido_members 结束

-- nfido_forums 开始


-- SQLINES DEMO *** finition
-- SQLINES LICENSE FOR EVALUATION USE ONLY
create sequence xmb_forums_seq;

create table xmb_forums
(
    type         varchar(15)  default ''        not null,
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
    password     varchar(32)  default ''        not null
)
;

-- SQLINES LICENSE FOR EVALUATION USE ONLY
create index displayorder
    on xmb_forums (displayorder);

-- SQLINES LICENSE FOR EVALUATION USE ONLY
create index fup
    on xmb_forums (fup);

-- SQLINES LICENSE FOR EVALUATION USE ONLY
create index status
    on xmb_forums (status);

-- SQLINES LICENSE FOR EVALUATION USE ONLY
create index type
    on xmb_forums (type);

-- nfido_forums 结束