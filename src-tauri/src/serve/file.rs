use anyhow::{anyhow, Result};
use tracing::info;

use crate::model::smov::SmovFile;
use crate::serve::smov_file::retrieve_all;
use std::{
  fs::{create_dir_all, read_dir, rename},
  path::PathBuf,
};
use tauri::api::file::Move;

lazy_static! {
  static ref EXT: Vec<&'static str> = vec!["srt", "ass", "lrc"];
}

pub struct TidySmov<'a> {
  pub id: &'a i64,
  pub name: &'a String,
}

impl TidySmov<'_> {
  pub async fn tidy(self: &Self) -> Result<PathBuf> {
    let tidy_path = &crate::app::APP.lock().conf.tidy_folder.clone();
    let smov_file = SmovFile::query_by_id(self.id).expect("查询数据库信息出现错误");

    let mut file_ch = "";
    if smov_file.isch.eq(&1) {
      file_ch = "-C";
    }

    let file_name = format!("{}.{}", &smov_file.realname, &smov_file.extension); //假设存在-C 保留-C
    let file_folder_path = PathBuf::from(&smov_file.path);
    let file_file_path = file_folder_path.join(&file_name);

    let tidy_folder_path = tidy_path.join(format!("{}{}", self.name, &file_ch));
    let tidy_file_noextension = format!("{}{}", &self.name, &file_ch);
    let tidy_name = format!("{}.{}", &tidy_file_noextension, &smov_file.extension);
    let tidy_file_path = &tidy_folder_path.join(&tidy_name);
    //判断文件是否还存在

    if !file_file_path.exists() {
      tracing::error!(message = "数据已被物理删除");
      return Err(anyhow!("Missing attribute: {}", "数据已被物理删除"));
    }
    info!("来源文件夹:{:?}", &file_file_path);
    info!("对象文件夹:{:?}", &tidy_file_path);
    //判断是否单文件
    if is_single(&smov_file.path) {
      //如果是单文件在整理目录新建文件夹 迁移视频文件
      if !&tidy_folder_path.exists() {
        create_dir_all(&tidy_folder_path).expect("创建视频文件夹错误");
      }

      // copy(&file_file_path, &tidy_file_path).expect("复制文件出现错误");
      let s = Move::from_source(&file_file_path);

      // let  test = OpenOptions::new().write (true).open(&tidy_folder_path);

      // rename(Path::new("C:Users\\Leri\\Videos\\cs\\1.txt"),Path::new("C:Users\\Leri\\Videos\\cs1\\2.txt"));

      // rename(&file_file_path, &tidy_file_path).unwrap();
       

      // let test = OpenOptions::new().write(true).open(&tidy_folder_path).await;
      // let test = test; //使用 rename 方法 转移文件 全部使用tokio

      // println!("{:?}",test.metadata()?.permissions());

      match s.to_dest(&tidy_file_path) {
        Err(err) => {
          tracing::error!(message = format!("移动文件出现错误:{}", err).as_str());
          return Err(anyhow!("移动文件出现错误:{}", err));
        }
        _ => {}
      };

      // remove_file(&file_file_path).expect("删除原文件出现错误");
    } else {
      //如果不是单文件，移动文件夹并重命名
      // copy(&file_folder_path, &tidy_folder_path).expect("复制文件夹出现错误");

      let s = Move::from_source(&file_folder_path);

      match s.to_dest(&tidy_folder_path) {
        Err(err) => return Err(anyhow!("移动文件出现错误:{}", err)),
        _ => {}
      };

      //重命名文件
      rename(tidy_folder_path.join(&file_name), tidy_file_path).expect("重命名文件出现错误");
      //尝试重命名字幕及zz文件
      if let Ok(entries) = read_dir(&tidy_folder_path) {
        for entry in entries {
          let srt_path = entry.expect("读取文件夹下文件出现错误").path();
          let exten = match srt_path.extension() {
            Some(ex) => ex,
            None => break,
          }
          .to_str()
          .unwrap_or_else(|| "");
          if EXT.contains(&exten) {
            let tidy_ext = format!("{}.{}", &self.name, exten);
            rename(srt_path, tidy_folder_path.join(tidy_ext)).expect("重命名文件出现错误");
          }
        }
      }
      // remove_dir(&file_folder_path).expect("删除原文件夹出现错误");
    }

    let img_path = tidy_folder_path.join("img");
    //创建img文件夹
    if !&img_path.join("detail").exists() {
      create_dir_all::<_>(&img_path.join("detail")).expect("创建视频文件夹错误");
    }

    SmovFile::update_path_name(
      self.id,
      tidy_file_noextension,
      tidy_folder_path.to_str().unwrap_or_else(|| "").to_string(),
    )
    .expect("更新数据库出现了错误！,现在没有处理错误，凉凉");

    Ok(img_path) //tidy_folder_path
  }
}

///需要优化是否单文件的检索，这种检索模式太难受了
fn is_single(path: &String) -> bool {
  let all = retrieve_all(path);
  if all.len() > 1 {
    true
  } else {
    false
  }
}
