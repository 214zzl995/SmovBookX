use std::path::PathBuf;
use std::thread;

use crate::model::file::SmovFile;
use crate::model::file::SmovPl;
use crate::model::file_seek::RetrievingSmov;
use crate::model::file_seek::SmovFileSeek;
use crate::model::folder::Folder;
use crate::model::smov::Smov;
use crate::response::response::Response;
use crate::serve::smov;
use crate::serve::smov_file;
use crate::serve::smov_file::SmovFileBack;
use crate::util::smov_format::SmovName;
use serde_json::Map;
use serde_json::Value;
use tauri::command;
use tauri::Manager;
use tauri::Window;

//检索新文件到数据库
#[command]
pub async fn query_new_file_todb() -> Response<Option<SmovFileBack>> {
  match smov_file::smov_file() {
    Ok(e) => Response::ok(Some(e), "检索成功"),
    Err(e) => Response::err(None, format!("{}", &e).as_str()),
  }
}

#[command]
pub async fn retrieve_data(retrieving_smov: RetrievingSmov) -> Response<Option<i32>> {
  let format = SmovName::format_smov_name(&retrieving_smov.seek_name);
  let handle = match thread::Builder::new()
    .name(String::from(&retrieving_smov.seek_name))
    .spawn(move || {
      let msg = format!("开始检索{}", &retrieving_smov.seek_name);
      tracing::info!(target: "frontend_log",message = msg.as_str());
      let s = tauri::async_runtime::block_on(async move {
        let a = smov::retrieve_smov(format, retrieving_smov.smov_id).await;
        tracing::info!("{}", "线程检索结束");
        return a;
      });
      let msg = format!("检索结束{}", &retrieving_smov.seek_name);
      tracing::info!(target: "frontend_log",message = msg.as_str());
      return s;
    }) {
    Ok(res) => res,
    Err(e) => return Response::err(None, format!("{}", &e).as_str()),
  };

  let handle = handle.join();
  match handle {
    Ok(e) => match e {
      Ok(n) => match n {
        true => {
          SmovFileSeek::change_status(retrieving_smov.id, 1).expect("出现了一个错误");
          Response::new(200, Some(1), "success")
        }
        false => {
          SmovFileSeek::change_status(retrieving_smov.id, 2).expect("出现了一个错误");
          Response::new(404, Some(1), "not found")
        }
      },
      Err(err) => {
        SmovFileSeek::change_status(retrieving_smov.id, 2).expect("出现了一个错误");
        Response::err(None, format!("{}", &err).as_str())
      }
    },
    Err(err) => {
      tracing::error!(message = format!("{:?}", err).as_str());
      Response::err(None, "出现了一个我不会解析的错误！")
    }
  }
}

//查找所有未被检索的数据
#[command]
pub fn query_unretrieved() -> Response<Option<Vec<SmovFile>>> {
  // info!("such information");
  match SmovFile::query_db_file_id_unseek() {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

#[command]
pub fn update_seekname(id: i32, seek_name: String) -> Response<Option<usize>> {
  match SmovFile::update_seekname(id, seek_name) {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

///需要对这个做一个判定 判定一 是否存在父文件夹 如果存在父文件夹 需要提示添加错误 存在父文件夹
#[command]
pub fn insert_folder(path: String) -> Response<Option<i32>> {
  let paths = PathBuf::from(&path);
  if paths.exists() {
    match Folder::insert_folder(path) {
      Ok(e) => return Response::new(200, Some(e), "success"),
      Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
    }
  } else {
    return Response::new(300, None, "目录不存在");
  }
}

#[command]
pub fn query_folder() -> Response<Option<Vec<Folder>>> {
  match Folder::query_folder() {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

#[command]
pub fn delete_folder(id: i32) -> Response<Option<bool>> {
  match Folder::delete_folder(id) {
    Ok(_) => return Response::new(200, Some(true), "删除成功"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

#[command]
pub async fn get_all_smov() -> Response<Option<Vec<Smov>>> {
  //检索文件夹 还是放到这里吧
  match Smov::get_all_smov() {
    Ok(res) => {
      return Response::new(200, Some(res), "success");
    }
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

#[command]
pub async fn get_smov_by_id(id: i64) -> Response<Option<Smov>> {
  match Smov::get_smov_by_id(id) {
    Ok(res) => return Response::new(200, Some(res), "success"),
    Err(err) => {
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str());
    }
  }
}

#[command]
pub async fn change_seek_status(
  smov: Vec<RetrievingSmov>,
  window: Window,
) -> Response<Option<bool>> {
  let mut to_smov = smov.clone();

  match SmovFileSeek::change_seek_status(&mut to_smov) {
    Ok(_) => {
      window
        .emit_to("seek", "addTask", &to_smov)
        .expect("向另一个窗口传送数据出现错误");

      return Response::new(200, Some(true), "success");
    }
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

#[command]
pub async fn get_seek_smov() -> Response<Option<Vec<RetrievingSmov>>> {
  match SmovFileSeek::get_seek_smov() {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

#[command]
pub async fn remove_smov_seek_status(id: Vec<i64>) -> Response<Option<bool>> {
  match SmovFileSeek::remove_smov_seek_status(id) {
    Ok(_) => return Response::new(200, Some(true), "success"),
    Err(err) => {
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str());
    }
  }
}

#[command]
pub async fn disable_smov(id: Vec<SmovPl>) -> Response<Option<bool>> {
  match SmovFile::disable(id) {
    Ok(_) => return Response::new(200, Some(true), "success"),
    Err(err) => {
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str());
    }
  }
}

#[command]
pub async fn change_active_status(id: i64, status: i32) -> Response<Option<bool>> {
  match SmovFile::change_active_status(id, status) {
    Ok(_) => return Response::new(200, Some(true), "success"),
    Err(err) => {
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str());
    }
  }
}

#[command]
pub async fn delete_smov(id: Vec<i64>) -> Response<Option<bool>> {
  match SmovFile::delete_smov(id) {
    Ok(_) => return Response::new(200, Some(true), "success"),
    Err(err) => {
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str());
    }
  }
}

#[command]
pub async fn get_setting_data() -> Response<Option<Map<String, Value>>> {
  let mut inner_map = Map::new();

  match Folder::query_folder() {
    Ok(vec) => {
      let vec = match serde_json::to_value(vec) {
        Ok(res) => res,
        Err(err) => {
          return Response::new(300, None, format!("反序列化错误：{}", err).as_str());
        }
      };
      inner_map.insert("seek_folder".to_string(), vec)
    }
    Err(err) => {
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str());
    }
  };

  let conf = &crate::app::APP.lock().conf;

  let conf = match serde_json::to_value(conf.clone()) {
    Ok(res) => res,
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  };

  inner_map.insert("conf".to_string(), conf);

  let hfs_config = &crate::app::HFSCONFIG.lock().clone();

  let hfs_config = match serde_json::to_value(hfs_config.clone()) {
    Ok(res) => res,
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  };

  inner_map.insert("hfs_config".to_string(), hfs_config);

  Response::new(200, Some(inner_map), "设置信息获取成功")
}

#[command]
pub async fn convert_smov2hls(id: i64) -> Response<Option<String>> {
  let _smov = match Smov::get_smov_by_id(id) {
    Ok(res) => res,
    Err(err) => {
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str());
    }
  };

  // smov.to_hls().unwrap();
  Response::ok(None, "获取成功")
}
