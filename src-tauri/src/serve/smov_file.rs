use parking_lot::MutexGuard;
use serde::{Deserialize, Serialize};

use crate::model::file::SmovFile;
use crate::model::folder::Folder;
use crate::model::smov::{Smov};
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::hash::Hash;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[warn(non_upper_case_globals)]
const FILE_TYPE: &'static [&'static str] = &["mp4", "flv", "mkv", "wmv"];

const SUB_TYPE: &'static [&'static str] = &["srt", "ass"];

#[derive(Hash, Debug, Deserialize, Serialize)]
pub struct SmovFileBack {
  time: i64,
  add_size: usize,
  del_smov_file: Vec<SmovFile>,
}

pub fn smov_file() -> Result<SmovFileBack> {
  let begin = timestamp(SystemTime::now());

  let mut folders = Folder::query_folder().unwrap();

  let tidy_folder = crate::app::APP.lock();

  let tidy_folder_path = match tidy_folder.conf.tidy_folder.as_os_str().to_str() {
    Some(res) => res,
    None => "",
  };

  //判定 当整理文件夹已经存在于队列中时 不添加 方法 tidy_in_vec是实现方法
  if !tidy_folder_path.eq("") && tidy_in_vec(tidy_folder_path, &folders) {
    folders.insert(
      0,
      Folder {
        id: 0,
        path: tidy_folder_path.to_string(),
      },
    );
  }

  MutexGuard::unlock_fair(tidy_folder); //这里必须对内存锁的部分锁定

  let db_smov: HashSet<SmovFile> = SmovFile::query_db_file_unid()
    .unwrap()
    .into_iter()
    .collect();

  let mut file_smov = Vec::new();

  for folder in folders {
    let main_path = folder.path;

    let mut loa_smov = retrieve_all(&main_path);

    file_smov.append(&mut loa_smov);
  }

  let file_smov: HashSet<SmovFile> = file_smov.into_iter().collect();

  let smov = file_smov.difference(&db_smov).collect::<Vec<&SmovFile>>();

  let smov_del = db_smov.difference(&file_smov).collect::<Vec<&SmovFile>>();

  let smov_del = match SmovFile::query_by_path_name(smov_del) {
    Ok(res) => res,
    Err(err) => {
      tracing::error!(message = format!("生成已删除文件出现错误:{}", err).as_str());
      return Err(anyhow!("生成已删除文件出现错误:{}", err));
    }
  };

  SmovFile::insert_file_data(&smov).unwrap();

  let end = timestamp(SystemTime::now());

  Ok(SmovFileBack {
    time: end - begin,
    add_size: smov.len(),
    del_smov_file: smov_del,
  })
}

fn timestamp(time: SystemTime) -> i64 {
  let since_the_epoch = time
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  let ms = since_the_epoch.as_secs() as i64 * 1000i64
    + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
  ms
}

fn is_mov_type(extension: &String) -> bool {
  let mut flag = false;
  for val in FILE_TYPE.iter() {
    if extension.eq(val) {
      flag = true;
      break;
    }
  }
  flag
}

fn is_sub_type(extension: &OsStr) -> bool {
  let mut flag = false;
  let extension = &extension.to_str().unwrap_or_else(|| "").to_string();
  for val in SUB_TYPE.iter() {
    if val.eq(extension) {
      flag = true;
      break;
    }
  }
  flag
}

///判断是否汉化放到这里
pub fn retrieve_all(path: &String) -> Vec<SmovFile> {
  let mut smovs: Vec<SmovFile> = Vec::new();

  if let Ok(entries) = fs::read_dir(&path) {
    for entry in entries {
      if let Ok(entry) = entry {
        let mat = entry.metadata().expect("无法读取文件元数据");
        if mat.is_file() {
          let realname = entry.file_name().into_string().expect("无法读取文件名称");
          let extension = match Path::new(&realname).extension() {
            Some(x) => x.to_os_string().into_string().expect("读取文件时发生错误"),
            None => String::from("None"), // None
          };

          let name = Path::new(&realname)
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .expect("读取文件时发生错误");
          let _format = name.to_uppercase().replace("-C", "").replace("-", "");
          if is_mov_type(&extension) {
            let file_name = String::from(&name);
            let isch = match file_name.contains("-c")
              || file_name.contains("-C")
              || file_name.contains("-ch")
            {
              true => 1,
              false => 0,
            };
            let res = SmovFile {
              id: 0,
              realname: String::from(&name),
              seekname: String::from(""),
              path: String::from(path), //对path多次引用 所以方法传入链接而不是 原始参数 在这里新建一个String 传入构造 链接就是前面带&
              len: mat.len(),
              created: timestamp(mat.created().expect("文件创建日期读取错误")),
              modified: timestamp(mat.modified().expect("文件修改日期读取错误")),
              extension,
              format: String::from(""),
              isch,
              is_active: 0,
            };
            smovs.push(res);
          }
        } else {
          let now_path = entry
            .path()
            .into_os_string()
            .into_string()
            .expect("读取文件时发生错误");
          let mut _loa_smov = retrieve_all(&now_path);
          smovs.append(&mut _loa_smov);
        }
      }
    }
  }
  smovs
}

pub fn tidy_in_vec(tidy_path: &str, folders: &Vec<Folder>) -> bool {
  let tidy = Path::new(tidy_path);
  for folder in folders {
    if tidy.starts_with(Path::new(&folder.path)) {
      return false;
    }
  }
  true
}

impl Smov {
  pub fn get_smov_img(self: &mut Self) -> Result<()> {
    let path = Path::new(&self.path).join("img");
    let img = path.join(format!("thumbs_{}.jpg", self.name));
    if img.exists() {
      self.thumbs_img = img.to_str().unwrap_or_else(|| "").to_string();
    };

    let img = path.join(format!("main_{}.jpg", self.name));
    if img.exists() {
      self.main_img = img.to_str().unwrap_or_else(|| "").to_string();
    };

    let path = path.join("detail");

    if path.exists() {
      let details = match path.read_dir() {
        Ok(res) => res,
        Err(err) => {
          tracing::error!(
            message = format!(
              "获取'{}'详情图片路径出现错误：'{}',请根据错误原因排查",
              self.name, err
            )
            .as_str()
          );
          return Err(anyhow!("Missing attribute: {}", err));
        }
      };
      for detail in details {
        match detail {
          Ok(res) => self.detail_img.insert(
            0,
            res
              .path()
              .as_os_str()
              .to_str()
              .unwrap_or_else(|| "")
              .to_string(),
          ),
          _ => {}
        }
      }
    }
    Ok(())
  }

  pub fn get_smov_sub_title(self: &mut Self) -> Result<()> {
    let path = Path::new(&self.path);

    let items = match path.read_dir() {
      Ok(res) => res,
      Err(err) => {
        tracing::error!(
          message = format!(
            "获取'{}'详情根目录路径出现错误：'{}',请根据错误原因排查",
            self.name, err
          )
          .as_str()
        );
        return Err(anyhow!("Missing attribute: {}", err));
      }
    };

    for item in items {
      match item {
        Ok(res) => {
          let extension = res.path();
          let extension = extension.extension().unwrap_or_else(|| &OsStr::new(""));

          if is_sub_type(extension) {
            self.sub_title.insert(
              0,
              res
                .file_name()
                .as_os_str()
                .to_str()
                .unwrap_or_else(|| "")
                .to_string(),
            )
          }
          
        }

        _ => {}
      }
    }

    Ok(())
  }

  pub fn get_smov_thumbs_img(self: &mut Self) -> Result<()> {
    let path = Path::new(&self.path).join("img");
    let img = path.join(format!("thumbs_{}.jpg", self.name));
    if img.exists() {
      self.thumbs_img = img.to_str().unwrap_or_else(|| "").to_string();
    };

    Ok(())
  }
}
