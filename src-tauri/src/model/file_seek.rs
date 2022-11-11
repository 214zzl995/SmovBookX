use rusqlite::{params, Error, Result};
use serde::{Deserialize, Serialize};

use super::connect::exec;

#[derive(Hash, Debug, Clone, Deserialize, Serialize)]
pub struct SmovFileSeek {
  pub id: i64,
  pub realname: String,  //当前的实际名称
  pub path: String,      //路径
  pub extension: String, //拓展名
  pub format: String,    //格式化后名称
}

#[derive(Hash, Debug, Deserialize, Serialize, Clone)]
pub struct RetrievingSmov {
  pub id: i64,
  pub smov_id: i64,
  pub seek_name: String,
  pub status: i32,
}


impl SmovFileSeek {
  pub fn change_seek_status(smov: &mut Vec<RetrievingSmov>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;

      for y in smov {
        tx.execute(
          "insert into seek_queue(smov_id, seekName) values (?1,?2)",
          params![y.smov_id, y.seek_name],
        )
        .expect("添加索引队列出现错误！");

        y.id = tx
          .query_row_and_then(
            "SELECT id from seek_queue where smov_id = ?1",
            params![&y.smov_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        y.status = 0;
      }

      tx.commit().expect("添加索引队列出现错误！");

      Ok(())
    })
  }

  pub fn get_seek_smov() -> Result<Vec<RetrievingSmov>> {
    exec(|conn| {
      let mut stmt = conn.prepare("SELECT id,smov_id,seekName,status FROM seek_queue")?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(RetrievingSmov {
          id: row.get(0)?,
          smov_id: row.get(1)?,
          seek_name: row.get(2)?, //当前的实际名称
          status: row.get(3)?,
        })
      })?;

      let mut res: Vec<RetrievingSmov> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn remove_smov_seek_status(id: Vec<i64>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      for y in id {
        match tx.execute("delete from seek_queue where id = ?1", params![y]) {
          Ok(_) => {}
          Err(err) => return Err(err),
        };
      }
      tx.commit()
    })
  }

  pub fn change_status(id: i64, status: i64) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      match tx.execute(
        "update seek_queue set status = ?1 where id = ?2",
        params![status, id],
      ) {
        Ok(_) => match tx.commit() {
          Ok(_) => Ok(()),
          Err(_) => return Err(Error::ExecuteReturnedResults),
        },
        Err(_) => return Err(Error::ExecuteReturnedResults),
      }
    })
  }
}
