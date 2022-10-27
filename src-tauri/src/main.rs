#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

#[macro_use]
extern crate lazy_static;

mod app;
mod cmd;
mod crawler;
mod hfs;
mod media;
mod model;
mod response;
mod serve;
mod task;
mod util;
mod window;

#[tokio::main]
async fn main() {
  app::lock_single();
  let app = tauri::Builder::default()
    .setup(|app| {
      app::listen_single_app(app.handle());
      app.manage(task::pool::pool_new(app.app_handle()).unwrap());
      task::pool::init_poll_window(app.app_handle());
      if cfg!(target_os = "windows") {
        app::webview2_is_installed(app);
      }
      if !app::init_app_dir() {
        tracing::error!("工作目录初始化失败！");
        panic!("工作目录初始化失败！");
      }
      if !app::init_app_log(app) {
        tracing::error!("日志系统初始化失败！");
        panic!("日志系统初始化失败！");
      }
      if !app::init_hfs() {
        tracing::error!("文件服务器配置初始化错误！");
        panic!("文件服务器配置初始化错误！");
      }
      app::init_app_shadows(app);
      model::smov::SMOVBOOK::init().expect("数据库初始化出现错误");
      Ok(())
    })
    .menu(app::create_app_menu())
    .on_menu_event(app::handle_event_app_menu_event)
    .system_tray(app::create_try())
    .on_system_tray_event(app::handle_system_tray_event)
    .invoke_handler(tauri::generate_handler![
      app::listen_single,
      cmd::tauri_cmd::log_operation,
      cmd::tauri_cmd::perform_request,
      cmd::cmd::query_unretrieved,
      cmd::cmd::query_new_file_todb,
      cmd::cmd::update_seekname,
      cmd::cmd::insert_folder,
      cmd::cmd::query_folder,
      cmd::cmd::retrieve_data,
      cmd::cmd::get_all_smov,
      cmd::cmd::change_seek_status,
      cmd::cmd::get_seek_smov,
      cmd::cmd::disable_smov,
      cmd::cmd::change_active_status,
      cmd::cmd::delete_smov,
      cmd::cmd::remove_smov_seek_status,
      cmd::cmd::get_smov_by_id,
      cmd::cmd::get_setting_data,
      cmd::cmd::delete_folder,
      cmd::cmd::convert_smov2hls,
      cmd::tauri_cmd::open_folder_select,
      cmd::tauri_cmd::test,
      cmd::tauri_cmd::open_in_explorer,
      cmd::tauri_cmd::update_tidy_folder,
      cmd::tauri_cmd::set_focus,
      cmd::tauri_cmd::create_new_window,
      cmd::tauri_cmd::set_style,
      cmd::tauri_cmd::get_local_ip,
      cmd::tauri_cmd::go_seek,
      cmd::tauri_cmd::go_task_pool,
      cmd::tauri_cmd::go_detail,
      cmd::tauri_cmd::change_seek_suspended,
      cmd::tauri_cmd::change_seek_shadow,
      hfs::axum_hfs::run_hfs,
      crawler::crawler::smov_crawler,
      task::pool::add_task_convert,
      task::pool::add_task_crawler,
      task::pool::pause_pool,
      task::pool::get_task_pool
    ])
    .build(tauri::generate_context!())
    .expect("error while running tauri application"); //这里要做错误处理 当出现错误时 用windows自带的弹窗 弹出错误信息

  app.run(app::handle_app_event);
}
