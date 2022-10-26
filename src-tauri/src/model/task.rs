use std::{collections::HashMap, path::PathBuf, time::Duration};

use rusqlite::{params, Connection, Result};

use crate::task::pool::{TaskAsk, TaskEvent, TaskStatus, TaskType};

fn create_sqlite_connection() -> Result<Connection> {
  let database = PathBuf::from(&crate::app::APP.lock().app_dir).join("SmovBook.db");
  let conn = Connection::open(database)?;
  conn.busy_timeout(Duration::new(15, 0))?;
  Ok(conn)
}
/// 封装一个方法，获取连接
pub fn exec<F, T>(func: F) -> Result<T>
where
  F: FnOnce(&mut Connection) -> Result<T>,
{
  match create_sqlite_connection() {
    Ok(mut conn) => func(&mut conn),
    Err(e) => Err(e),
  }
}

pub struct TasksModel {
  pub id: i64,
  pub name: String,
  pub uuid: String,
  pub smov_id: i64,
  pub task_type: TaskType,
  pub task_status: TaskStatus,
}

impl TasksModel {
  pub fn from_task_event(task_event: &TaskEvent, uuid: &String) -> Result<Self> {
    Ok(Self {
      id: 0,
      name: task_event.ask.name.clone(),
      uuid:uuid.clone(),
      smov_id: task_event.ask.id,
      task_type: task_event.event_type.clone(),
      task_status: task_event.status.clone(),
    })
  }

  pub fn to_task_event(self: &Self) -> Result<TaskEvent> {
    Ok(TaskEvent {
      event_type: self.task_type.clone(),
      ask: TaskAsk {
        id: self.smov_id,
        name: self.name.clone(),
      },
      status: self.task_status.clone(),
    })
  }
  pub fn insert(self: &Self) -> Result<()> {
    exec(|conn| {
      conn
        .execute(
          "insert into tasks(name, uuid, smov_id, type, status) values(?1,?2,?3,?4,?5)",
          params![
            self.name,
            self.uuid,
            self.smov_id,
            self.task_type.clone().to_i64(),
            self.task_status.clone().to_i64()
          ],
        )
        .expect("插入tasks表出现错误");

      Ok(())
    })
  }
  pub fn get_all_vec() -> Result<Vec<TaskEvent>> {
    exec(|conn| {
      let mut stmt = conn.prepare("select id,name,uuid,smov_id,type,status from tasks")?;
      let tasks: Vec<TaskEvent> = stmt
        .query_map([], |row| {
          Ok(
            TasksModel {
              id: row.get(0)?,
              name: row.get(1)?,
              uuid: row.get(2)?,
              smov_id: row.get(3)?,
              task_type: TaskType::to_enum(row.get(4)?),
              task_status: TaskStatus::to_enum(row.get(5)?),
            }
            .to_task_event()
            .unwrap(),
          )
        })?
        .map(|task_event| task_event.unwrap())
        .collect();
      Ok(tasks)
    })
  }
}

pub fn get_all_task() -> Result<HashMap<String, TaskEvent>> {
  let mut task_event: HashMap<String, TaskEvent> = HashMap::new();
  exec(|conn| {
    let mut stmt = conn.prepare("select id,name,uuid,smov_id,type,status from tasks")?;
    let _ = stmt.query_map([], |row| {
      let task_model = TasksModel {
        id: row.get(0)?,
        name: row.get(1)?,
        uuid: row.get(2)?,
        smov_id: row.get(3)?,
        task_type: TaskType::to_enum(row.get(4)?),
        task_status: TaskStatus::to_enum(row.get(5)?),
      }
      .to_task_event()
      .unwrap();
      task_event.insert(row.get(2)?, task_model);
      Ok(())
    })?;
    Ok(task_event)
  })
}
