use std::{collections::HashMap, sync::Arc};

use parking_lot::{Mutex, MutexGuard};
use tracing::info;
use window_shadows::set_shadow;

use crate::{
  crawler::crawler::smov_crawler_program_pool,
  model::{
    smov::{RetrievingSmovPool, Smov},
    task::TasksModel,
  },
  response::response::Response,
  util::smov_format::SmovName,
};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager, Window, WindowUrl};
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

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct NextTask {
  task_event: TaskEvent,
  uuid: String,
}

#[derive(Eq, Hash, PartialEq, Clone, Deserialize, Serialize, Debug)]
pub struct TaskEvent {
  pub id: i64,
  pub event_type: TaskType,
  pub ask: TaskAsk,
  pub status: TaskStatus,
}

pub struct Task<'a> {
  task_event: &'a TaskEvent,
  app_handle: &'a AppHandle,
  uuid: String,
  smov_pool: SmovPool,
}

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct TaskAsk {
  pub id: i64,
  pub name: String,
}

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct TaskMessage<T> {
  uuid: String,
  msg: String,
  data: T,
}

#[derive(Eq, Hash, PartialEq, Clone, Deserialize, Serialize, Debug)]
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

impl TaskStatus {
  pub fn to_i64(self: Self) -> i64 {
    match self {
      TaskStatus::Wait => 0,
      TaskStatus::Running => 1,
      TaskStatus::Fail => 2,
      TaskStatus::Success => 3,
    }
  }

  pub fn to_enum(flag: i64) -> Self {
    match flag {
      0 => TaskStatus::Wait,
      1 => TaskStatus::Running,
      2 => TaskStatus::Fail,
      3 => TaskStatus::Success,
      _ => TaskStatus::Wait,
    }
  }
}

impl TaskType {
  pub fn to_i64(self: Self) -> i64 {
    match self {
      TaskType::Crawler => 0,
      TaskType::Convert => 1,
    }
  }

  pub fn to_enum(flag: i64) -> Self {
    match flag {
      0 => TaskType::Crawler,
      1 => TaskType::Convert,
      _ => TaskType::Crawler,
    }
  }
}

pub fn init_poll_window(app_handle: AppHandle) {
  let window = Window::builder(&app_handle, "TaskPool", WindowUrl::App("TaskPool".into()))
    .title("任务列表")
    .center()
    .min_inner_size(700.0, 500.0)
    .decorations(false)
    .skip_taskbar(false)
    .resizable(true)
    .transparent(true)
    .visible(false)
    .build()
    .unwrap();

  set_shadow(&window, true).unwrap();
}

pub fn pool_new(app_handle: AppHandle) -> Result<SmovPool, PoolErr> {
  let thread_num = crate::app::APP.lock().conf.thread.clone();
  let tasks = crate::model::task::get_all_task().unwrap();
  match Builder::new_multi_thread().build() {
    Ok(pool) => Ok(Arc::new(Mutex::new(TaskPool {
      pool,
      tasks,
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

  let mut task = TaskEvent::new(task_type.clone(), task_ask).unwrap();

  let id = TasksModel::from_task_event(&task, &uuid)
    .unwrap()
    .insert()
    .unwrap();

  task.set_id(id);

  task_pool_lock.tasks.insert(uuid.clone(), task.clone());

  let task_size = task_pool_lock.exec_num.get(&task_type).unwrap();

  //将任务存入数据库

  if task_size < &task_pool_lock.thread_num && task_pool_lock.can_run() {
    let now_size = task_pool_lock.exec_num.get(&task_type).unwrap().clone();

    task_pool_lock
      .exec_num
      .insert(task_type.clone(), now_size + 1);

    let task_pool_copy = task_pool.clone();

    let task = task_type_run(task_pool_copy, task_type.clone());

    task_pool_lock.pool.spawn(task);
  }

  uuid
}

async fn task_type_run(smov_pool: SmovPool, task_type: TaskType) {
  //当当前有下一个线程且 可以运行时 while 这里的 next 应该就由while 取 不然可能会出现被别人抢了的情况 一般不会 所以先放着不管了 不然又得大改
  while let (Some(next_task), true) = (
    {
      let pool = smov_pool.lock();
      let next_task = pool.get_next_task(&task_type).clone();
      MutexGuard::unlock_fair(pool);
      next_task
    },
    {
      let pool = smov_pool.lock();
      let can_run = pool.can_run().clone();
      MutexGuard::unlock_fair(pool);
      can_run
    },
  ) {
    let uuid = next_task.uuid;
    info!("正在执行程序{}", uuid);
    task_run(smov_pool.clone(), uuid);
  }
  let mut pool = smov_pool.lock();

  let task_size = pool.exec_num.get(&task_type.clone()).unwrap().clone();

  pool.exec_num.insert(task_type.clone(), task_size - 1);

  if pool.get_exec_all_num().clone() == 0 && pool.status.eq(&PoolStatus::Running) {
    //当线程已经结束 没有下一个线程 且当前状态为正在运行时 且运行的线程数为0 更新池的状态 为等待
    pool.status = PoolStatus::Idle;
  }
}

fn task_run(smov_pool: SmovPool, uuid: String) {
  let pool = smov_pool.lock();

  let task_event = pool.tasks.get(&uuid).unwrap().clone();
  let app_handle = &pool.app_handle.clone();

  //生成task
  let task = Task {
    task_event: &task_event,
    app_handle,
    uuid,
    smov_pool: smov_pool.clone(),
  };

  MutexGuard::unlock_fair(pool);

  task.join();
}

impl TaskPool {
  pub fn get_exec_all_num(self: &Self) -> i64 {
    let mut exec_num = 0;

    for (_, value) in self.exec_num.iter() {
      exec_num = exec_num + value;
    }
    exec_num
  }

  //获取下一位时就需要 将这个任务占用 建议直接改状态
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
      id: 0,
      event_type: task_type,
      ask: task_ask,
      status: TaskStatus::Wait,
    })
  }

  fn set_id(self: &mut Self, id: i64) {
    self.id = id;
  }
}

impl TaskEvent {}

impl<T> TaskMessage<T> {
  pub fn new(uuid: String, data: T, msg: &str) -> TaskMessage<T> {
    TaskMessage {
      uuid,
      data,
      msg: msg.to_string(),
    }
  }
}

impl Task<'_> {
  pub fn emit_status(self: &Self, task_status: TaskStatus, msg: &str) {
    let mut pool = self.smov_pool.lock();
    let mut task = pool.tasks.get(&self.uuid).unwrap().clone();

    task.status = task_status.clone();

    pool.tasks.insert(self.uuid.clone(), task);

    MutexGuard::unlock_fair(pool);

    //修改数据库 运行的状态 ，将状态与消息传入数据库

    self
      .app_handle
      .emit_all(
        "TASKPOOL://status_change",
        TaskMessage::new(self.uuid.clone(), task_status, msg),
      )
      .unwrap();
  }

  pub fn emit_schedule(self: &Self, schedule: i64, msg: &str) {

    //将进度消息传入msg 每一个消息都应对应一个进度或状态

    self
      .app_handle
      .emit_all(
        "TASKPOOL://schedule_change",
        TaskMessage::new(self.uuid.clone(), schedule, msg),
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
          self.emit_status(TaskStatus::Fail, "数据库中未找到资源");
          return;
        }
      }
      .unwrap();

      if let Err(err) = smov.to_hls(&self) {
        self.emit_status(TaskStatus::Fail, &err.to_string());
      }
    } else if event_type.eq(&TaskType::Crawler) {
      let id = self.task_event.ask.id;
      let retrieving_smov = RetrievingSmovPool::get_retriecing_smov_by_id(id.clone()).unwrap();

      let format = SmovName::format_smov_name(&retrieving_smov.seek_name);

      if let Err(err) = smov_crawler_program_pool(format, id, &self) {
        self.emit_status(TaskStatus::Fail, &err.to_string());
      }
    }
  }
}

#[command]
pub fn add_task_convert(
  task_pool: tauri::State<SmovPool>,
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
  task_pool: tauri::State<SmovPool>,
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
pub fn pause_pool(task_pool: tauri::State<SmovPool>) {
  task_pool.lock().pause();
}

#[command]
pub fn get_task_pool(
  task_pool: tauri::State<SmovPool>,
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
