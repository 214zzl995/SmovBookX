use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::{connect::exec, file_seek::SmovFileSeek};

#[derive(Hash, Debug, Clone, Serialize, Deserialize)]
pub struct SmovFile {
  pub id: i64,
  pub realname: String, //当前的实际名称
  pub seekname: String,
  pub path: String,      //路径
  pub len: u64,          //大小
  pub created: i64,      //本地创建时间
  pub modified: i64,     //本地修改时间
  pub extension: String, //拓展名
  pub format: String,    //格式化后名称
  pub isch: i32,
  pub is_active: i32,
}

#[derive(Hash, Debug, Deserialize, Serialize)]
pub struct SmovPl {
  id: i64,
  is_active: i64,
}

impl PartialEq for SmovFile {
  fn eq(&self, other: &Self) -> bool {
    (self.realname == other.realname)
      && (self.path == other.path)
      && (self.extension == other.extension)
  }
}

impl Eq for SmovFile {}

impl SmovFile {
  pub fn insert_file_data(smov_file: &Vec<&SmovFile>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;

      for y in smov_file {
        let format = crate::util::smov_format::SmovName::format_smov_name(&y.realname);
        tx.execute(
            "insert into smov(realname, path, len, created, modified, extension, format,seekname,isch) select ?1,?2,?3,?4,?5,?6,?7,?8,?9 where not exists(select * from smov where realname = ?10 and path = ?11)",
            params![y.realname,y.path,y.len,y.created,y.modified,y.extension,format,y.realname,y.isch,y.realname,y.path],
            ).expect("插入smov表出现错误");
      }

      tx.commit().unwrap();

      Ok(())
    })
  }

  //正常的标准写法！
  pub fn disable(id: Vec<SmovPl>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      for y in id {
        match tx.execute(
          "update smov set is_active = ?1 where id = ?2",
          params![y.is_active, y.id],
        ) {
          Ok(_) => {}
          Err(err) => return Err(err),
        };
      }
      tx.commit()
    })
  }

  pub fn change_active_status(id: i64, status: i32) -> Result<()> {
    exec(|conn| {
      match conn.execute(
        "update smov set is_active = ?1 where id = ?2",
        params![status, id],
      ) {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
      }
    })
  }

  pub fn query_db_file_unid() -> Result<Vec<SmovFile>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt = conn.prepare(
        "SELECT realname,seekname,path,len,created,modified,extension,format,isch FROM smov",
      )?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFile {
          id: 0,
          realname: row.get(0)?,      //当前的实际名称
          seekname: String::from(""), //当前的实际名称
          path: row.get(2)?,          //路径
          len: row.get(3)?,           //大小
          created: row.get(4)?,       //本地创建时间
          modified: row.get(5)?,      //本地修改时间
          extension: row.get(6)?,     //拓展名
          format: String::from(""),   //格式化后名称
          isch: row.get(8)?,
          is_active: 0,
        })
      })?;

      let mut res: Vec<SmovFile> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.expect("出现错误");
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn query_by_id(id: &i64) -> Result<SmovFile, rusqlite::Error> {
    exec(|conn| {
      conn.query_row_and_then("SELECT realname,seekname,path,len,created,modified,extension,format,isch,is_active FROM smov where id = ?1",
         params![id], |row| {
          Ok(SmovFile {
            id: 0,
            realname: row.get(0)?,      //当前的实际名称
            seekname: String::from(""), //当前的实际名称
            path: row.get(2)?,          //路径
            len: row.get(3)?,           //大小
            created: row.get(4)?,       //本地创建时间
            modified: row.get(5)?,      //本地修改时间
            extension: row.get(6)?,     //拓展名
            format: String::from(""),   //格式化后名称
            isch: row.get(8)?,
            is_active:row.get(9)?,
          })
        })
    })
  }

  pub fn _query_unseek_db_file() -> Result<Vec<SmovFileSeek>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt =
        conn.prepare("SELECT id,realname,path,extension,format FROM smov where is_retrieve = 0")?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFileSeek {
          id: row.get(0)?,
          realname: row.get(1)?,  //当前的实际名称
          path: row.get(2)?,      //路径
          extension: row.get(3)?, //拓展名
          format: row.get(4)?,    //格式化后名称
        })
      })?;

      let mut res: Vec<SmovFileSeek> = Vec::new();
      for smov_file in smov_file_iter {
        res.push(smov_file.unwrap());
      }
      Ok(res)
    })
  }

  pub fn query_db_file_id_unseek() -> Result<Vec<SmovFile>, rusqlite::Error> {
    // and is_active=1
    exec(|conn| {
      let mut stmt = conn.prepare(
        "SELECT id,realname, seekname,path,len,created,modified,extension,format,isch,is_active
            FROM smov
            where is_retrieve = 0 
              and not exists(select 1 from seek_queue where smov_id = smov.id)",
      )?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFile {
          id: row.get(0)?,
          realname: row.get(1)?,  //当前的实际名称
          seekname: row.get(2)?,  //当前的实际名称
          path: row.get(3)?,      //路径
          len: row.get(4)?,       //大小
          created: row.get(5)?,   //本地创建时间
          modified: row.get(6)?,  //本地修改时间
          extension: row.get(7)?, //拓展名
          format: row.get(8)?,    //格式化后名称
          isch: row.get(9)?,
          is_active: row.get(10)?,
        })
      })?;

      let mut res: Vec<SmovFile> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn _query_db_file_id() -> Result<Vec<SmovFile>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt = conn.prepare(
        "SELECT id,realname,seekname,path,len,created,modified,extension,format,isch,is_active FROM smov",
      )?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFile {
          id: row.get(0)?,
          realname: row.get(1)?,  //当前的实际名称
          seekname: row.get(2)?,  //当前的实际名称
          path: row.get(3)?,      //路径
          len: row.get(4)?,       //大小
          created: row.get(5)?,   //本地创建时间
          modified: row.get(6)?,  //本地修改时间
          extension: row.get(7)?, //拓展名
          format: row.get(8)?,    //格式化后名称
          isch: row.get(9)?,
          is_active: row.get(10)?,
        })
      })?;

      let mut res: Vec<SmovFile> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn update_seekname(id: i32, seek_name: String) -> Result<usize, rusqlite::Error> {
    exec(|conn| {
      let format = crate::util::smov_format::SmovName::format_smov_name(&seek_name);
      let update_size = conn
        .execute(
          "update smov set seekname = ?1 ,format = ?2 where id = ?3",
          params![seek_name, format, id],
        )
        .expect("更新出现错误");

      Ok(update_size)
    })
  }

  pub fn update_path_name(
    id: &i64,
    realname: String,
    path: String,
  ) -> Result<usize, rusqlite::Error> {
    exec(|conn| {
      let update_size = conn
        .execute(
          "update smov set realname = ?1 ,path = ?2 where id = ?3",
          params![realname, path, id],
        )
        .expect("更新出现错误");

      Ok(update_size)
    })
  }

  pub fn delete_smov(id: Vec<i64>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      for y in id {
        match tx.execute("delete from smov where id = ?1", params![y]) {
          Ok(_) => {}
          Err(err) => {
            tx.rollback().unwrap();
            return Err(err);
          }
        };
      }
      tx.commit()
    })
  }

  pub fn query_by_path_name(smov: Vec<&SmovFile>) -> Result<Vec<SmovFile>, rusqlite::Error> {
    let mut smovs: Vec<SmovFile> = Vec::new();
    exec(|conn| {
      let tx = conn.transaction()?;
      for y in smov {
        match tx.query_row_and_then(
          "select id,realname,path from smov where realname = ?1 and path = ?2",
          params![y.realname, y.path],
          |row| {
            Ok(SmovFile {
              id: row.get(0)?,
              realname: row.get(1)?,
              seekname: String::from(""),
              path: row.get(2)?,
              len: 0,
              created: 0,
              modified: 0,
              extension: String::from(""),
              format: String::from(""),
              isch: 0,
              is_active: 0,
            })
          },
        ) {
          Ok(res) => smovs.push(res),
          Err(err) => {
            tx.rollback().unwrap();
            return Err(err);
          }
        };
      }
      tx.commit().unwrap();

      Ok(smovs)
    })
  }
}
