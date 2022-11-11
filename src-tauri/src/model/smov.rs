use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use crate::hfs::res::{ListData, PageParams};

use super::connect::exec;

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Smov {
  pub id: i64,
  pub name: String,  //云端
  pub title: String, //标题
  pub path: String,  //路径
  pub realname: String,
  pub len: u64,             //大小
  pub created: i64,         //本地创建时间
  pub modified: i64,        //本地修改时间
  pub extension: String,    //拓展名
  pub format: String,       //格式化后名称
  pub release_time: String, //发行时间
  pub duration: i64,        //时长
  pub maker: String,        //商
  pub maker_id: i64,        //商
  pub publisher: String,    //方
  pub publisher_id: i64,    //方
  pub serie: String,        //系列
  pub serie_id: i64,        //系列
  pub director: String,     //导演
  pub director_id: i64,     //导演
  pub tags: Vec<Tag>,       //标签
  pub actors: Vec<Actor>,   //演员
  pub isch: bool,
  pub thumbs_img: String,
  pub main_img: String,
  pub detail_img: Vec<String>,
  pub sub_title: Vec<String>,
}

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Tag {
  id: i64,
  name: String,
}

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Actor {
  id: i64,
  name: String,
}



pub struct SMOVBOOK {
  _v: u64,
}

impl Smov {
  pub fn _insert_all(smov: Smov) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;

      tx.execute(
        "insert into maker(name) select ?1 where not exists (select * from maker where name= ?2)",
        params![smov.maker, smov.maker],
      )?;

      let maker: u64 = tx
        .query_row_and_then(
          "SELECT id from maker where name = ?1",
          params![smov.maker],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
        "insert into serie(name) select ?1 where not exists (select * from serie where name= ?2)",
        params![smov.serie, smov.serie],
      )
      .expect("插入出现错误？");

      let serie: u64 = tx
        .query_row_and_then(
          "SELECT id from serie where name = ?1",
          params![smov.serie],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into director(name) select ?1 where not exists (select * from director where name= ?2)",
                params![smov.director, smov.director],
                )?;

      let director: u64 = tx
        .query_row_and_then(
          "SELECT id from director where name = ?1",
          params![smov.director],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into publisher(name) select ?1 where not exists (select * from publisher where name= ?2)",
                params![smov.publisher, smov.publisher],
                )?;

      let publisher: u64 = tx
        .query_row_and_then(
          "SELECT id from publisher where name = ?1",
          params![smov.publisher],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into smov(name, path, len, created, modified, extension, format,publisher_id, makers_id, series_id, directors_id) select ?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11 where not exists(select * from smov where format = ?12)",
                params![smov.name, smov.path, smov.len, smov.created, smov.modified, smov.extension, smov.format,publisher,maker,serie,director,smov.format],
                ).expect("插入smov表出现错误");

      let _smovid: u64 = tx
        .query_row_and_then(
          "SELECT id from smov where format = ?1",
          params![smov.format],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      //插入tag 暂时没用问题不大
      // for tag in smov.tags {
      //   tx.execute(
      //     "insert into tag(name) select ?1 where not exists (select * from tag where name= ?2)",
      //     params![tag, tag],
      //   )?;

      //   let tagid: u64 = tx
      //     .query_row_and_then("SELECT id from tag where name = ?1", params![tag], |row| {
      //       row.get(0)
      //     })
      //     .expect("查询出现错误");

      //   tx.execute(
      //     "insert into smov_tag(smov_id,tag_id) values(?1,?2)",
      //     params![smovid, tagid],
      //   )?;
      // }

      //插入演员暂时没用
      // for actor in smov.actors {
      //   tx.execute(
      //     "insert into actor(name) select ?1 where not exists (select * from actor where name= ?2)",
      //     params![actor, actor],
      //   )?;

      //   let actorid: u64 = tx
      //     .query_row_and_then(
      //       "SELECT id from actor where name = ?1",
      //       params![actor],
      //       |row| row.get(0),
      //     )
      //     .expect("查询出现错误");

      //   tx.execute(
      //     "insert into smov_actor(smov_id,actor_id) values(?1,?2)",
      //     params![smovid, actorid],
      //   )?;
      // }
      tx.commit().unwrap();

      Ok(())
    })
  }

  pub fn get_all_smov() -> Result<Vec<Smov>> {
    exec(|conn| {
      let tx = conn.transaction()?;
      let mut stmt = tx.prepare(
        "select id, name,title, path, realname, len, created, modified, extension, 
        format, release_time, duration,makers_id, publisher_id, series_id, directors_id, 
         isch from smov where is_retrieve = 1",
      )?;
      let smov_iter = stmt.query_map([], |row| {
        Ok(Smov {
          id: row.get(0)?,
          name: row.get(1)?,
          title: row.get(2)?,
          path: row.get(3)?,
          realname: row.get(4)?,
          len: row.get(5)?,
          created: row.get(6)?,
          modified: row.get(7)?,
          extension: row.get(8)?,
          format: row.get(9)?,
          release_time: row.get(10)?,
          duration: row.get(11)?,
          maker: String::from(""),
          maker_id: row.get(12)?,
          publisher: String::from(""),
          publisher_id: row.get(13)?,
          serie: String::from(""),
          serie_id: row.get(14)?,
          director: String::from(""),
          director_id: row.get(15)?,
          tags: Vec::new(),
          actors: Vec::new(),
          isch: row.get(16)?,
          thumbs_img: String::from(""),
          main_img: String::from(""),
          detail_img: Vec::new(),
          sub_title: Vec::new(),
        })
      })?;
      let mut smov_list: Vec<Smov> = Vec::new();

      for smov_s in smov_iter {
        let mut smov = smov_s.expect("序列化错误");
        smov.maker = tx
          .query_row_and_then(
            "SELECT name from maker where id = ?1",
            params![&smov.maker_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.publisher = tx
          .query_row_and_then(
            "SELECT name from publisher where id = ?1",
            params![&smov.publisher_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.serie = tx
          .query_row_and_then(
            "SELECT name from serie where id = ?1",
            params![&smov.serie_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.director = tx
          .query_row_and_then(
            "SELECT name from director where id = ?1",
            params![&smov.director_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        let mut stmt = tx.prepare(
          "select tag.id,tag.name from tag,smov_tag where tag.id = smov_tag.tag_id and smov_tag.smov_id = ?1",
        )?;

        let tag_iter = stmt.query_map([smov.id], |row| {
          Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
          })
        })?;

        for tag in tag_iter {
          let tag1 = tag.unwrap();
          smov.tags.push(tag1);
        }

        let mut stmt = tx.prepare(
          "select actor.id,actor.name from actor,smov_actor where actor.id = smov_actor.actor_id and smov_actor.smov_id = ?1",
        )?;

        let actor_iter = stmt.query_map([smov.id], |row| {
          Ok(Actor {
            id: row.get(0)?,
            name: row.get(1)?,
          })
        })?;

        for actor in actor_iter {
          let actor1 = actor.unwrap();
          smov.actors.push(actor1);
        }

        smov.get_smov_thumbs_img().unwrap();

        smov_list.push(smov);
      }
      Ok(smov_list)
    })
  }

  pub fn get_smov_by_id(id: i64) -> Result<Smov> {
    exec(|conn| {
      let tx = conn.transaction()?;

      let mut smov = tx
        .query_row_and_then(
          "select id, name,title, path, realname, len, created, modified, extension, 
          format, release_time, duration,makers_id, publisher_id, series_id, directors_id, 
           isch from smov where id = ?1",
          params![id],
          |row| -> Result<Smov, rusqlite::Error> {
            Ok(Smov {
              id: row.get(0)?,
              name: row.get(1)?,
              title: row.get(2)?,
              path: row.get(3)?,
              realname: row.get(4)?,
              len: row.get(5)?,
              created: row.get(6)?,
              modified: row.get(7)?,
              extension: row.get(8)?,
              format: row.get(9)?,
              release_time: row.get(10)?,
              duration: row.get(11)?,
              maker: String::from(""),
              maker_id: row.get(12)?,
              publisher: String::from(""),
              publisher_id: row.get(13)?,
              serie: String::from(""),
              serie_id: row.get(14)?,
              director: String::from(""),
              director_id: row.get(15)?,
              tags: Vec::new(),
              actors: Vec::new(),
              isch: row.get(16)?,
              thumbs_img: String::from(""),
              main_img: String::from(""),
              detail_img: Vec::new(),
              sub_title: Vec::new(),
            })
          },
        )
        .unwrap();

      smov.maker = tx
        .query_row_and_then(
          "SELECT name from maker where id = ?1",
          params![&smov.maker_id],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      smov.publisher = tx
        .query_row_and_then(
          "SELECT name from publisher where id = ?1",
          params![&smov.publisher_id],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      smov.serie = tx
        .query_row_and_then(
          "SELECT name from serie where id = ?1",
          params![&smov.serie_id],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      smov.director = tx
        .query_row_and_then(
          "SELECT name from director where id = ?1",
          params![&smov.director_id],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      let mut stmt = tx.prepare(
          "select tag.id,tag.name from tag,smov_tag where tag.id = smov_tag.tag_id and smov_tag.smov_id = ?1",
        )?;

      let tag_iter = stmt.query_map([smov.id], |row| {
        Ok(Tag {
          id: row.get(0)?,
          name: row.get(1)?,
        })
      })?;

      for tag in tag_iter {
        let tag1 = tag.unwrap();
        smov.tags.push(tag1);
      }

      let mut stmt = tx.prepare(
          "select actor.id,actor.name from actor,smov_actor where actor.id = smov_actor.actor_id and smov_actor.smov_id = ?1",
        )?;

      let actor_iter = stmt.query_map([smov.id], |row| {
        Ok(Actor {
          id: row.get(0)?,
          name: row.get(1)?,
        })
      })?;

      for actor in actor_iter {
        let actor1 = actor.unwrap();
        smov.actors.push(actor1);
      }

      smov.get_smov_img().unwrap();
      smov.get_smov_sub_title().unwrap();

      Ok(smov)
    })
  }

  pub fn get_smov_pagination(page_params: PageParams) -> Result<ListData<Smov>> {
    let page_num = page_params.page_num.unwrap_or(0);
    let page_per_size = page_params.page_size.unwrap_or(10);
    exec(|conn| {
      let tx = conn.transaction()?;

      let sql = format!("select count(*) from smov where is_retrieve = 1");

      let total = tx
        .query_row_and_then(&sql, params![], |row| row.get(0))
        .expect("查询出现错误");

      let sql = format!(
        "select id, name,title, path, realname, len, created, modified, extension, 
      format, release_time, duration,makers_id, publisher_id, series_id, directors_id, 
       isch from smov where is_retrieve = 1 Limit {} Offset {}",
        &page_per_size,
        &page_num * &page_per_size
      );
      let mut stmt = tx.prepare(&sql)?;
      let smov_iter = stmt.query_map([], |row| {
        Ok(Smov {
          id: row.get(0)?,
          name: row.get(1)?,
          title: row.get(2)?,
          path: row.get(3)?,
          realname: row.get(4)?,
          len: row.get(5)?,
          created: row.get(6)?,
          modified: row.get(7)?,
          extension: row.get(8)?,
          format: row.get(9)?,
          release_time: row.get(10)?,
          duration: row.get(11)?,
          maker: String::from(""),
          maker_id: row.get(12)?,
          publisher: String::from(""),
          publisher_id: row.get(13)?,
          serie: String::from(""),
          serie_id: row.get(14)?,
          director: String::from(""),
          director_id: row.get(15)?,
          tags: Vec::new(),
          actors: Vec::new(),
          isch: row.get(16)?,
          thumbs_img: String::from(""),
          main_img: String::from(""),
          detail_img: Vec::new(),
          sub_title: Vec::new(),
        })
      })?;
      let mut list: Vec<Smov> = Vec::new();

      for smov_s in smov_iter {
        let mut smov = smov_s.expect("序列化错误");
        smov.maker = tx
          .query_row_and_then(
            "SELECT name from maker where id = ?1",
            params![&smov.maker_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.publisher = tx
          .query_row_and_then(
            "SELECT name from publisher where id = ?1",
            params![&smov.publisher_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.serie = tx
          .query_row_and_then(
            "SELECT name from serie where id = ?1",
            params![&smov.serie_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.director = tx
          .query_row_and_then(
            "SELECT name from director where id = ?1",
            params![&smov.director_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        let mut stmt = tx.prepare(
          "select tag.id,tag.name from tag,smov_tag where tag.id = smov_tag.tag_id and smov_tag.smov_id = ?1",
        )?;

        let tag_iter = stmt.query_map([smov.id], |row| {
          Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
          })
        })?;

        for tag in tag_iter {
          let tag1 = tag.unwrap();
          smov.tags.push(tag1);
        }

        let mut stmt = tx.prepare(
          "select actor.id,actor.name from actor,smov_actor where actor.id = smov_actor.actor_id and smov_actor.smov_id = ?1",
        )?;

        let actor_iter = stmt.query_map([smov.id], |row| {
          Ok(Actor {
            id: row.get(0)?,
            name: row.get(1)?,
          })
        })?;

        for actor in actor_iter {
          let actor1 = actor.unwrap();
          smov.actors.push(actor1);
        }

        smov.get_smov_thumbs_img().unwrap();

        list.push(smov);
      }
      let res = ListData {
        total,
        list,
        page_num,
        total_pages: (*&total as f32 / *&page_per_size as f32).ceil() as usize,
      };

      Ok(res)
    })
  }
}

impl SMOVBOOK {
  pub fn init() -> Result<()> {
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
