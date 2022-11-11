use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
use std::result::Result::Ok;

use super::connect::exec;

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
  pub id: i64,
  pub path: String,
}

impl Folder {
  pub fn insert_folder(path: String) -> Result<i32, rusqlite::Error> {
    exec(|conn| {
      conn.execute(
            "insert into sys_folder(path) select ?1 where not exists(select * from sys_folder where path = ?2)",
            params![path,path],
            ).expect("加入检索位置出现错误");

      let folder_id: i32 = conn
        .query_row_and_then(
          "SELECT id from sys_folder where path = ?1",
          params![path],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      Ok(folder_id)
    })
  }

  pub fn delete_folder(id: i32) -> Result<i32, rusqlite::Error> {
    exec(|conn| {
      conn
        .execute("delete from sys_folder where id =?1 ", params![id])
        .expect("删除检索文件夹出现错误");
      Ok(1)
    })
  }

  pub fn query_folder() -> Result<Vec<Folder>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt = conn.prepare("SELECT id,path FROM sys_folder")?;
      let folder_iter = stmt.query_map([], |row| {
        Ok(Folder {
          id: row.get(0)?,
          path: row.get(1)?,
        })
      })?;

      let mut res: Vec<Folder> = Vec::new();

      for smov_file in folder_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }
      Ok(res)
    })
  }
}
