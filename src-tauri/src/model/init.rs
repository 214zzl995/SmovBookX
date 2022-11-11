use rusqlite::{Connection, Result};

use super::connect::exec;
use include_dir::{include_dir, Dir};

static SQL_DIR: Dir = include_dir!("./sql");

pub struct SmovDb {
  v: f64,
}

impl SmovDb {
  pub fn new() -> Result<SmovDb> {
    let v = exec(|conn| Self::get_user_version(conn))?;
    Ok(SmovDb { v })
  }

  fn get_user_version(conn: &mut Connection) -> Result<f64> {
    let v = match conn.pragma_query_value(None, "user_version", |row| row.get::<_, f64>(0)) {
      Ok(v) => v,
      Err(e) => return Err(e.into()),
    };
    Ok(v)
  }

  pub fn init(self: &Self) -> Result<()> {
    let new_v = env!("DB-VERSION").parse::<f64>().unwrap();
    tracing::info!("当前需求数据库版本:{}", new_v);

    exec(|conn| {
      let now_v_begin = Self::get_user_version(conn)?;
      //如果版本为0 执行一次init 否则执行增量更新包
      if now_v_begin != 0.0 {
        //通过不断的获取当前的版本 执行新的sql文件
        while Self::get_user_version(conn)? < new_v {
          let now_v = Self::get_user_version(conn)?;
          tracing::info!("正在更新数据库版本:{}", now_v);
          let next_sql_file = SQL_DIR
            .get_file(format!("alter{}.sql", now_v))
            .unwrap()
            .clone();
          let next_sql = next_sql_file.contents_utf8().unwrap();
          conn.execute_batch(next_sql)?;
        }
      } else {
        tracing::info!("正在初始化数据库");
        let init_sql_file = SQL_DIR.get_file("init.sql").unwrap().clone();
        let init_sql = init_sql_file.contents_utf8().unwrap();
        conn.execute_batch(init_sql)?;
      }
      tracing::info!("当前数据库版本:{}", Self::get_user_version(conn)?);
      Ok(())
    })
  }

  pub fn init_old() -> Result<()> {
    exec(|conn| {
      conn
        .execute(
          "create table if not exists smov
            (
                id           integer primary key autoincrement,
                name         TEXT,
                title        TEXT,
                realname     TEXT,
                seekname     TEXT,
                path         TEXT,
                len          integer,
                created      integer,
                modified     integer,
                extension    TEXT,
                format       TEXT,
                release_time TEXT,
                publish_time TEXT,
                duration     integer,
                publisher_id integer,
                makers_id    integer              Null,
                series_id    integer              Null,
                directors_id integer              Null,
                is_retrieve  TINYINT(1) Default 0 Null,
                is_active    TINYINT(1) Default 1 Null,
                isch         TINYINT(1) Default 0 Null
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists actor
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists publisher
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists director
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists maker
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists serie
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists tag
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists smov_actor
            (
                id       integer primary key autoincrement,
                smov_id  integer,
                actor_id integer
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists smov_tag
            (
                id      integer primary key autoincrement,
                smov_id integer,
                tag_id  integer
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists sys_folder
            (
                id      integer primary key autoincrement,
                path    TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "create table if not exists tasks
            (
                id      integer primary key autoincrement,
                name    TEXT,
                uuid    TEXT,
                smov_id integer,
                type    integer,
                status  integer default 0
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "create table if not exists tasks_msg
            (
                id      integer primary key autoincrement,
                tasks_id integer,
                msg     TEXT
            )",
          [],
        )
        .unwrap();

      Ok(())
    })
  }
}
