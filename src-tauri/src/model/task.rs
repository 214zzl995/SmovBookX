use std::collections::HashMap;

use rusqlite::{params, Result};

use crate::task::pool::{TaskAsk, TaskEvent, TaskStatus, TaskType};

use super::connect::exec;

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
      uuid: uuid.clone(),
      smov_id: task_event.ask.id,
      task_type: task_event.event_type.clone(),
      task_status: task_event.status.clone(),
    })
  }

  pub fn to_task_event(self: &Self) -> Result<TaskEvent> {
    Ok(TaskEvent {
      id: self.id,
      event_type: self.task_type.clone(),
      ask: TaskAsk {
        id: self.smov_id,
        name: self.name.clone(),
      },
      status: self.task_status.clone(),
    })
  }

  pub fn insert(self: &Self) -> Result<i64> {
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

      let task_id: i64 = conn
        .query_row_and_then(
          "SELECT id from tasks where uuid = ?1",
          params![self.uuid],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      Ok(task_id)
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

pub fn get_all_task_old_and_bad() -> Result<HashMap<String, TaskEvent>> {
  let mut task_events: HashMap<String, TaskEvent> = HashMap::new();
  exec(|conn| {
    let mut stmt = conn.prepare("select id,name,uuid,smov_id,type,status from tasks")?;
    let _ = stmt.query_map([], |row| {
      let task_event = TasksModel {
        id: row.get(0)?,
        name: row.get(1)?,
        uuid: row.get(2)?,
        smov_id: row.get(3)?,
        task_type: TaskType::to_enum(row.get(4)?),
        task_status: TaskStatus::to_enum(row.get(5)?),
      }
      .to_task_event()
      .unwrap();

      task_events.insert(row.get(2)?, task_event);
      Ok(())
    })?;
    Ok(task_events)
  })
}

pub fn get_all_task() -> Result<HashMap<String, TaskEvent>> {
  let mut task_events: HashMap<String, TaskEvent> = HashMap::new();
  exec(|conn| {
    let mut stmt = conn.prepare("select id,name,uuid,smov_id,type,status from tasks")?;
    let models = stmt.query_map([], |row| {
      Ok(TasksModel {
        id: row.get(0)?,
        name: row.get(1)?,
        uuid: row.get(2)?,
        smov_id: row.get(3)?,
        task_type: TaskType::to_enum(row.get(4)?),
        task_status: TaskStatus::to_enum(row.get(5)?),
      })
    })?;

    for model in models {
      let model = model?;
      let uuid = model.uuid.clone();
      task_events.insert(uuid, model.to_task_event().unwrap());
    }

    Ok(task_events)
  })
}
