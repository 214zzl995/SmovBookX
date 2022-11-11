create table if not exists smov (
    id integer primary key autoincrement,
    name TEXT,
    title TEXT,
    realname TEXT,
    seekname TEXT,
    path TEXT,
    len integer,
    created integer,
    modified integer,
    extension TEXT,
    format TEXT,
    release_time TEXT,
    publish_time TEXT,
    duration integer,
    publisher_id integer,
    makers_id integer Null,
    series_id integer Null,
    directors_id integer Null,
    is_retrieve TINYINT(1) Default 0 Null,
    is_active TINYINT(1) Default 1 Null,
    isch TINYINT(1) Default 0 Null
);
Create Table if not exists actor (
    id integer primary key autoincrement,
    name TEXT
);
Create Table if not exists publisher (
    id integer primary key autoincrement,
    name TEXT
);
Create Table if not exists director (
    id integer primary key autoincrement,
    name TEXT
);
Create Table if not exists maker (
    id integer primary key autoincrement,
    name TEXT
);
Create Table if not exists serie (
    id integer primary key autoincrement,
    name TEXT
);
Create Table if not exists tag (
    id integer primary key autoincrement,
    name TEXT
);
Create Table if not exists smov_actor (
    id integer primary key autoincrement,
    smov_id integer,
    actor_id integer
);
Create Table if not exists smov_tag (
    id integer primary key autoincrement,
    smov_id integer,
    tag_id integer
);
Create Table if not exists sys_folder (
    id integer primary key autoincrement,
    path TEXT
);
create table if not exists tasks (
    id integer primary key autoincrement,
    name TEXT,
    uuid TEXT,
    smov_id integer,
    type integer,
    status integer default 0
);
create table if not exists tasks_msg (
    id integer primary key autoincrement,
    tasks_id integer,
    msg TEXT
);

pragma user_version = 20221109;