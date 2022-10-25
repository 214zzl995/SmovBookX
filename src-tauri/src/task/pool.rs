use std::{collections::HashMap, sync::Arc};

use parking_lot::{Mutex, MutexGuard};

use crate::{
  crawler::crawler::smov_crawler_program_pool,
  model::smov::{RetrievingSmovPool, Smov},
  response::response::Response,
  util::smov_format::SmovName,
};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager};
use thiserror::Error;
use tokio::runtime::{Builder, Runtime};
use uuid::Uuid;

pub struct TaskPool {
  pub pool: Runtime,
  pub tasks: HashMap<String, TaskEvent>,
  pub exec_num: HashMap<TaskType, i64>,
  pub thread_num: i64,
  pub status: PoolStatus,
  pub app_handle: AppHandle,
}

pub struct NextTask {
  task_event: TaskEvent,
  uuid: String,
}

#[derive(Eq, Hash, PartialEq, Clone, Deserialize, Serialize)]
pub struct TaskEvent {
  pub event_type: TaskType,
  pub ask: TaskAsk,
  pub status: TaskStatus,
}

pub struct Task<'a> {
  task_event: &'a TaskEvent,
  app_handle: &'a AppHandle,
  uuid: String,
}

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Clone)]
pub struct TaskAsk {
  pub id: i64,
  pub name: String,
}

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct TaskMessage<T> {
  uuid: String,
  data: T,
}

#[derive(Eq, Hash, PartialEq, Clone, Deserialize, Serialize)]
pub enum TaskType {
  Crawler,
  Convert,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum PoolStatus {
  Running, //正在运行
  Pause,   //暂停
  Idle,    //空闲
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum TaskStatus {
  Wait,    //等待
  Running, //正在运行
  Fail,    //失败
  Success, //成功
}

#[derive(Error, Debug)]
pub enum PoolErr {
  #[error("线程池创建失败！")]
  PoolCreateError(String),
}

#[derive(Error, Debug)]
pub enum TaskErr {
  #[error("获取主数据失败")]
  _NotFound,
  #[error("unknown error")]
  _Unknown,
}

pub type SmovPool = Arc<Mutex<TaskPool>>;

pub fn pool_new(app_handle: AppHandle) -> Result<SmovPool, PoolErr> {
  let thread_num = crate::app::APP.lock().conf.thread.clone();
  match Builder::new_multi_thread().build() {
    Ok(pool) => Ok(Arc::new(Mutex::new(TaskPool {
      pool,
      tasks: HashMap::new(),
      exec_num: {
        let mut map = HashMap::new();
        map.insert(TaskType::Convert, 0);
        map.insert(TaskType::Crawler, 0);
        map
      },
      thread_num,
      status: PoolStatus::Idle,
      app_handle,
    }))),
    Err(err) => Err(PoolErr::PoolCreateError(err.to_string())),
  }
}

fn pool_add_task(task_pool: SmovPool, task_ask: TaskAsk, task_type: TaskType) -> String {
  let mut task_pool_lock = task_pool.lock();
  let uuid = Uuid::new_v4().to_string();

  let task = TaskEvent::new(task_type.clone(), task_ask).unwrap();

  task_pool_lock.tasks.insert(uuid.clone(), task);

  let task_size = task_pool_lock.exec_num.get(&task_type).unwrap();

  //将任务存入数据库

  if task_size < &task_pool_lock.thread_num && task_pool_lock.can_run() {
    let now_size = task_pool_lock.exec_num.get(&task_type).unwrap().clone();

    task_pool_lock.exec_num.insert(task_type, now_size + 1);

    let task_pool_copy = task_pool.clone();

    let task = task_run(task_pool_copy, uuid.clone());

    task_pool_lock.pool.spawn(task);
  }

  uuid
}

async fn task_run(smov_pool: SmovPool, uuid: String) {
  let pool = smov_pool.lock();

  let task_event = pool.tasks.get(&uuid).unwrap().clone();
  let app_handle = &pool.app_handle.clone();

  //生成task
  let task = Task {
    task_event: &task_event,
    app_handle,
    uuid,
  };

  MutexGuard::unlock_fair(pool);

  task.join();

  let mut pool = smov_pool.lock();

  let task_size = pool.exec_num.get(&task_event.event_type).unwrap().clone();
  pool
    .exec_num
    .insert(task_event.event_type.clone(), task_size - 1);

  //判断是否有下一位
  if let (Some(next_task), true) = (pool.get_next_task(&task_event.event_type), pool.can_run()) {
    //继续执行下一条数据
    let lock_pool = smov_pool.clone();
    let task_run_fn = task_run(lock_pool, next_task.uuid);

    if false {
      pool.pool.spawn(task_run_fn);
    }
  } else {
    //判断是否还有正在运行的线程
    if pool.get_exec_all_num() == 0 {
      //当线程已经结束 没有下一个线程 且运行的线程数为0 更新池的状态 为等待
      pool.status = PoolStatus::Idle;
    }
  }
}

impl TaskPool {
  pub fn new(app_handle: AppHandle) -> Result<Self, PoolErr> {
    let thread_num = crate::app::APP.lock().conf.thread.clone();
    match Builder::new_multi_thread().build() {
      Ok(pool) => Ok(TaskPool {
        pool,
        tasks: HashMap::new(),
        exec_num: {
          let mut map = HashMap::new();
          map.insert(TaskType::Convert, 0);
          map.insert(TaskType::Crawler, 0);
          map
        },
        thread_num,
        status: PoolStatus::Idle,
        app_handle,
      }),
      Err(err) => Err(PoolErr::PoolCreateError(err.to_string())),
    }
  }

  //该方法已弃用
  pub async fn run(self: &mut Self, uuid: String) {
    let mut task_evenet = self.tasks.get(&uuid).unwrap().clone();

    //执行程序

    //更新task的状态
    //task_evenet.status = task_status;
    task_evenet.status = TaskStatus::Success;
    self.tasks.insert(uuid, task_evenet);

    //判断是否有下一个task
    if let (Some(_task), true) = (self.get_next_task(&TaskType::Convert), self.can_run()) {
      //给pool 塞入下一个
    } else {
      self.exec_num.insert(
        TaskType::Convert,
        self.exec_num.get(&TaskType::Convert).unwrap() - 1,
      );

      //判断是否还有正在运行的线程
      if self.get_exec_all_num() == 0 {}
    }
  }

  pub fn get_exec_all_num(self: &Self) -> i64 {
    let mut exec_num = 0;

    for (_, value) in self.exec_num.iter() {
      exec_num = exec_num + value;
    }
    exec_num
  }

  pub fn get_next_task(self: &Self, task_type: &TaskType) -> Option<NextTask> {
    for (key, value) in self.tasks.iter() {
      if value.status.eq(&TaskStatus::Wait) && value.event_type.eq(task_type) {
        return Some(NextTask {
          task_event: value.clone(),
          uuid: key.clone(),
        });
      }
    }
    None
  }

  pub fn pause(self: &mut Self) {
    self.status = PoolStatus::Pause;
  }

  pub fn can_run(self: &Self) -> bool {
    self.status.eq(&PoolStatus::Idle) || self.status.eq(&PoolStatus::Running)
  }
}

impl TaskEvent {
  fn new(task_type: TaskType, task_ask: TaskAsk) -> Result<Self, TaskErr> {
    Ok(TaskEvent {
      event_type: task_type,
      ask: task_ask,
      status: TaskStatus::Wait,
    })
  }
}

impl TaskEvent {}

impl<T> TaskMessage<T> {
  fn new(uuid: String, data: T) -> Self {
    Self { uuid, data }
  }
}

impl Task<'_> {
  pub fn emit_status(self: &Self, task_status: TaskStatus) {
    self
      .app_handle
      .emit_all(
        "TASKPOOL://status_change",
        TaskMessage::new(self.uuid.clone(), task_status),
      )
      .unwrap();
  }

  //进度还需要 msg消息
  pub fn emit_schedule(self: &Self, schedule: i64) {
    self
      .app_handle
      .emit_all(
        "TASKPOOL://schedule_change",
        TaskMessage::new(self.uuid.clone(), schedule),
      )
      .unwrap();
  }

  //根据类型运行程序
  pub fn join(self: &Self) {
    let event_type = self.task_event.event_type.clone();
    if event_type.eq(&TaskType::Convert) {
      let smov = match Smov::get_smov_by_id(self.task_event.ask.id) {
        Ok(smov) => Some(smov),
        Err(_) => {
          self.emit_status(TaskStatus::Fail);
          return;
        }
      }
      .unwrap();
      smov.to_hls(&self).unwrap();
    } else if event_type.eq(&TaskType::Crawler) {
      let id = self.task_event.ask.id;
      let retrieving_smov = RetrievingSmovPool::get_retriecing_smov_by_id(id.clone()).unwrap();

      let format = SmovName::format_smov_name(&retrieving_smov.seek_name);

      smov_crawler_program_pool(format, id, &self).unwrap();
    }
  }
}

#[command]
pub fn add_task_convert(
  task_pool: tauri::State<Arc<Mutex<TaskPool>>>,
  task_ask: TaskAsk,
) -> Response<Option<String>> {
  let task_pool: SmovPool = task_pool.inner().clone();
  Response::ok(
    Some(pool_add_task(task_pool, task_ask, TaskType::Convert)),
    "成功",
  )
}

#[command]
pub fn add_task_crawler(
  task_pool: tauri::State<Arc<Mutex<TaskPool>>>,
  task_ask: TaskAsk,
) -> Response<Option<String>> {
  let task_pool: SmovPool = task_pool.inner().clone();
  //需要发送消息给taskpool界面 让界面能新增一个任务 所以应该要返回一个taskEvent
  Response::ok(
    Some(pool_add_task(task_pool, task_ask, TaskType::Crawler)),
    "成功",
  )
}

#[command]
pub fn pause_pool(task_pool: tauri::State<Arc<Mutex<TaskPool>>>) {
  task_pool.lock().pause();
}

#[command]
pub fn get_task_pool(
  task_pool: tauri::State<Arc<Mutex<TaskPool>>>,
) -> Response<HashMap<TaskType, HashMap<String, TaskEvent>>> {
  //获取当前所有数据
  let tasks = task_pool.lock().tasks.clone();

  let mut crawler = HashMap::new();
  let mut convert = HashMap::new();

  for (key, value) in tasks.into_iter() {
    if value.event_type.eq(&TaskType::Convert) {
      convert.insert(key, value);
    } else {
      crawler.insert(key, value);
    }
  }

  let mut task_map = HashMap::new();
  task_map.insert(TaskType::Convert, convert);
  task_map.insert(TaskType::Crawler, crawler);
  Response::ok(task_map, "成功")
}

//重构seek界面
