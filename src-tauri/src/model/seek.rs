use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::connect::exec;

#[derive(Debug, Deserialize, Serialize)]
pub struct SmovSeek {
  pub id: i64,
  pub name: String,         //云端
  pub title: String,        //标题
  pub format: String,       //格式化后名称
  pub release_time: String, //发行时间
  pub duration: i32,        //时长
  pub publishers: String,   //方
  pub makers: String,       //商
  pub series: String,       //系列
  pub directors: String,    //导演
  pub tags: Vec<String>,    //标签
  pub actors: Vec<String>,  //演员
}

impl SmovSeek {
  pub fn insert_by_path_name(self: &Self) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      tx.execute(
        "insert into maker(name) select ?1 where not exists (select * from maker where name= ?2)",
        params![self.makers, self.makers],
      )?;

      let maker: u64 = tx
        .query_row_and_then(
          "SELECT id from maker where name = ?1",
          params![self.makers],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
        "insert into serie(name) select ?1 where not exists (select * from serie where name= ?2)",
        params![self.series, self.series],
      )
      .expect("插入出现错误？");

      let serie: u64 = tx
        .query_row_and_then(
          "SELECT id from serie where name = ?1",
          params![self.series],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into director(name) select ?1 where not exists (select * from director where name= ?2)",
                params![self.directors, self.directors],
                )?;

      let director: u64 = tx
        .query_row_and_then(
          "SELECT id from director where name = ?1",
          params![self.directors],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into publisher(name) select ?1 where not exists (select * from publisher where name= ?2)",
                params![self.publishers, self.publishers],
                )?;

      let publisher: u64 = tx
        .query_row_and_then(
          "SELECT id from publisher where name = ?1",
          params![self.publishers],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "update smov set name = ?1 ,makers_id =?2,series_id = ?3,directors_id =?4 , 
                publisher_id = ?5,duration = ?6,release_time = ?7 , is_retrieve = ?8 ,title =?9  where id = ?10;",
                params![self.name,maker,serie,director,publisher,self.duration,self.release_time,1,self.title,self.id],
                ).expect("插入smov表出现错误");

      for tag in &self.tags {
        tx.execute(
          "insert into tag(name) select ?1 where not exists (select * from tag where name= ?2)",
          params![tag, tag],
        )?;

        let tagid: u64 = tx
          .query_row_and_then("SELECT id from tag where name = ?1", params![tag], |row| {
            row.get(0)
          })
          .expect("查询出现错误");

        tx.execute(
          "insert into smov_tag(smov_id,tag_id) values(?1,?2)",
          params![self.id, tagid],
        )?;
      }

      for actor in &self.actors {
        tx.execute(
          "insert into actor(name) select ?1 where not exists (select * from actor where name= ?2)",
          params![actor, actor],
        )?;

        let actorid: u64 = tx
          .query_row_and_then(
            "SELECT id from actor where name = ?1",
            params![actor],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        tx.execute(
          "insert into smov_actor(smov_id,actor_id) values(?1,?2)",
          params![self.id, actorid],
        )?;
      }

      tx.commit().unwrap();

      Ok(())
    })
  }
}
