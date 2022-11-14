use crate::serve::file::TidySmov;
use rusqlite::{params, Result};
use super::connect::exec;

impl TidySmov<'_> {
  pub fn get_name_count(self: &Self) -> Result<i32> {
    exec(|conn| {
      conn.query_row_and_then(
        "select count(*) from smov where name =  = ?1",
        params![self.name],
        |row| row.get(0),
      )
    })
  }
}
