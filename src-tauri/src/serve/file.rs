use crate::model::smov::SmovFile;
use crate::serve::smov_file::retrieve_all;
use std::{
  fs::{copy, create_dir_all, remove_dir, remove_file},
  path::PathBuf,
};

pub struct tidy_smov<'a> {
  pub id: &'a i64,
  pub name: &'a String,
}

impl tidy_smov<'_> {
  pub fn tidy(self: &Self) {
    let tidy_path = &crate::app::CONF.lock().tidy_folder.clone();
    let smov_file = SmovFile::query_by_id(self.id).expect("查询数据库信息出现错误");
    let file_name = format!("{}.{}", &smov_file.realname, &smov_file.extension);
    let file_folder_path = PathBuf::from(&smov_file.path);
    let file_file_path = file_folder_path.join(file_name);
    let tidy_folder_path = tidy_path.join(self.name);
    let tidy_name = format!("{}.{}", &self.name, &smov_file.extension);
    let tidy_file_path = &tidy_folder_path.join(&tidy_name);
    //判断文件是否还存在
    if !file_file_path.exists() {
      println!("{}", "数据物理删除了");
      return;
    }
    println!("file_folder_path:{:?}", &file_file_path);
    println!("tidy_folder_path:{:?}", &tidy_file_path);
    //判断是否单文件
    if is_single(&smov_file.path) {
      //如果是单文件在整理目录新建文件夹 迁移视频文件
      if !&tidy_folder_path.exists() {
        create_dir_all::<_>(&tidy_folder_path).expect("创建整理文件夹错误");
      }
      copy(&file_file_path, &tidy_file_path).expect("复制文件出现错误");
      remove_file(&file_file_path).expect("删除原文件出现错误");
    } else {
      //如果不是单文件，移动文件夹并重命名
      copy(&file_folder_path, &tidy_folder_path).expect("复制文件夹出现错误");
      //重命名文件

      //尝试重命名字幕及zz文件
    }
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
