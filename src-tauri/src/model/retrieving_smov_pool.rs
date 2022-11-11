use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::connect::exec;

#[derive(Hash, Debug, Deserialize, Serialize, Clone)]
pub struct RetrievingSmovPool {
  pub id: i64,
  pub seek_name: String,
}

impl RetrievingSmovPool {
  pub fn get_retriecing_smov_by_id(id: i64) -> Result<Self, rusqlite::Error> {
    exec(|conn| {
      conn.query_row_and_then(
        "SELECT id ,seekname FROM smov where id = ?1",
        params![id],
        |row| {
          Ok(RetrievingSmovPool {
            id: row.get(0)?,
            seek_name: row.get(1)?,
          })
        },
      )
    })
  }
}
