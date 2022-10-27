use serde::Deserialize;
use std::fs::{write, File};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use window_shadows::set_shadow;
use window_vibrancy::{
  apply_acrylic, apply_blur, apply_mica, clear_acrylic, clear_blur, clear_mica,
};

use tauri::api::dialog;
use tauri::{command, LogicalSize, Manager, PhysicalPosition, Position, Size, Window, WindowUrl};

extern crate toml;

use crate::app::Conf;
use crate::response::response::Response;
use crate::window::window::{set_effect, Effect};

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  MyCustomCommand { argument: String },
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[command]
pub fn log_operation(event: String, payload: Option<String>) {
  println!("{} {:?}", event, payload);
}

#[command]
pub fn perform_request(endpoint: String, body: RequestBody) -> String {
  println!("{} {:?}", endpoint, body);
  "message response".into()
}

#[command]
pub async fn create_new_window(label: String, effect: String, path: String, window: Window) {
  match window.get_window(&label) {
    Some(win) => set_focus(label, win),
    None => {
      let window = Window::builder(&window, &label.clone(), WindowUrl::App(path.into()))
        .focus()
        .title(label.clone())
        .center()
        .inner_size(30.0, 30.0)
        .decorations(false)
        .build()
        .unwrap();

      set_shadow(&window, true).unwrap();
      clear_blur(&window).unwrap();
      clear_acrylic(&window).unwrap();
      clear_mica(&window).unwrap();

      match effect.as_str() {
        "blur" => apply_blur(&window, Some((238, 238, 244, 100))).unwrap(),
        "acrylic" => apply_acrylic(&window, Some((238, 238, 244, 100))).unwrap(),
        "mica" => apply_mica(&window).unwrap(),
        _ => (),
      };
    }
  };
}

#[command]
pub async fn go_seek(window: Window) {
  match window.get_window("seek") {
    Some(win) => {
      // win.emit_all("seek_single", "").unwrap();
      win.unminimize().unwrap();
      win.set_focus().unwrap();
      win.show().unwrap();
    }
    None => {
      let window = Window::builder(&window, "seek", WindowUrl::App("seek".into()))
        .focus()
        .title("检索列表")
        .center()
        .inner_size(400.0, 700.0)
        .decorations(false)
        .skip_taskbar(false)
        .resizable(false)
        .transparent(true)
        .build()
        .unwrap();

      set_shadow(&window, true).unwrap();
    }
  };
}

#[command]
pub async fn go_task_pool(window: Window) {
  match window.get_window("TaskPool") {
    Some(win) => {
      win.unminimize().unwrap();
      win.set_focus().unwrap();
      win.show().unwrap();
    }
    None => {
      let window = Window::builder(&window, "TaskPool", WindowUrl::App("TaskPool".into()))
        .title("任务列表")
        .center()
        .inner_size(700.0, 600.0)
        .decorations(false)
        .skip_taskbar(false)
        .resizable(false)
        .transparent(true)
        .build()
        .unwrap();

      set_shadow(&window, true).unwrap();
    }
  };
}

#[command]
pub async fn change_seek_suspended(flag: bool, window: Window) {
  let position = window.current_monitor().unwrap();

  let position = position.unwrap();
  //方向错误
  //不应该用显示与否去做交互
  //窗口不应该做显示隐藏
  //而是应该在当前的悬浮球上 从左上角做一个 水波动画 就可以避免这些问题 而且对于交互来说更加科学一点

  // 方案一 窗口隐藏加载 但是点击会不跟手
  // window.hide().unwrap();
  // std::thread::sleep(std::time::Duration::from_millis(200));

  // 方案二：窗口大小设置为0 不会造成残影问题
  // let phy = Size::Logical(LogicalSize {
  //   width: 1.0,
  //   height: 1.0,
  // });
  // window.set_size(phy).unwrap();

  match flag {
    true => {
      let position = Position::Logical(
        PhysicalPosition {
          x: position.size().width - (position.size().width / 8), //8
          y: position.size().height / 8,                          //8
        }
        .to_logical(position.scale_factor()),
      );

      let phy = Size::Logical(LogicalSize {
        width: 60.0,  //50
        height: 40.0, //30
      });

      set_shadow(&window, false).unwrap();
      std::thread::sleep(std::time::Duration::from_millis(170));
      window.set_position(position).unwrap();
      window.set_size(phy).unwrap();

      window.set_skip_taskbar(true).unwrap();
      window.set_always_on_top(true).unwrap();
      //window.show().unwrap();
    }
    false => {
      std::thread::sleep(std::time::Duration::from_millis(170));

      // 根据点击位置获取方法
      // let x_flag =
      //   x < (position.size().width as f64 / position.scale_factor().ceil() / 2.0).ceil() as u32;

      // let y_flag =
      //   y < (position.size().height as f64 / position.scale_factor().ceil() / 2.0).ceil() as u32;

      // let position = Position::Logical(LogicalPosition {
      //   x: {
      //     match x_flag {
      //       true => x as f64,
      //       false => (x - 400) as f64,
      //     }
      //   },
      //   y: {
      //     match y_flag {
      //       true => y as f64,
      //       false => (y - 800) as f64,
      //     }
      //   },
      // });

      // println!(
      //   "x:{},y:{},x_flag:{},y_flag:{},w:{},ws:{}",
      //   window.inner_position().unwrap().x,
      //   window.inner_position().unwrap().y,
      //   x_flag,
      //   y_flag,
      //   position.size().width,
      //   position.size().width as f64 / position.scale_factor()
      // );

      //根据窗口位置获取
      let window_position = window.outer_position().unwrap();

      let x_flag = window_position.x < (position.size().width / 2) as i32;

      let y_flag = window_position.y < (position.size().height / 2) as i32;

      let mut logical = window_position.to_logical(position.scale_factor());

      if !x_flag {
        logical.x = logical.x - 400.0 + 60.0;
      };

      if !y_flag {
        logical.y = logical.y - 800.0 + 40.0;
      }

      let position = Position::Logical(logical);

      //判断当前点击的位置
      // let x_flag = x <

      //因为先设置了大小 然后修改了位置 且不管显示隐藏都会有一个动画的过程 导致双击悬浮球会出现残影 windows的动画导致闪屏
      window
        .set_size(Size::Logical(LogicalSize {
          width: 400.0,
          height: 800.0,
        }))
        .unwrap();

      window.set_position(position).unwrap();

      set_shadow(&window, true).unwrap();

      window.set_skip_taskbar(false).unwrap();
      window.set_always_on_top(false).unwrap();
    }
  }
}

#[command]
pub async fn change_seek_shadow(window: Window) {
  std::thread::sleep(std::time::Duration::from_millis(250));
  set_shadow(&window, true).unwrap();
}

#[command]
pub async fn go_detail(label: String, url: String, window: Window, app: tauri::AppHandle) {
  match window.get_window(&label) {
    Some(win) => set_focus(label, win),
    None => {
      let window = Window::builder(&app, String::from(&label), WindowUrl::App(url.into()))
        .focus()
        .title(String::from(&label))
        .center()
        .decorations(false)
        .transparent(true)
        .visible(false)
        .min_inner_size(800.0, 600.0)
        .build()
        .unwrap();

      //反正有问题 模仿main的处理方式 解决一下
      window.show().unwrap();

      set_shadow(&window, true).unwrap();

      //现在半透明有bug 先不用了
      //set_effect(Effect::Acrylic, &window);
    }
  };
}

#[command]
pub fn open_folder_select() -> Response<String> {
  let (sender, receiver) = channel::<PathBuf>();
  let a = dialog::FileDialogBuilder::new();
  a.pick_folder(move |path| {
    if path.is_some() {
      sender.send(path.unwrap()).unwrap()
    } else {
      drop(sender)
    }
  });

  let maybe_received = receiver.recv();
  if maybe_received.is_err() {
    return Response::err(String::from("No folder selected"), "No folder selected");
  }

  let foo = maybe_received.unwrap();
  let maybe_str = pathbuf_to_string(foo);
  if maybe_str.is_none() {
    return Response::err(String::from("Invalid folder path"), "Invalid folder path");
  }

  let selection_str = maybe_str.unwrap();

  Response::ok(selection_str, "成功")
}

//在资源管理器中打开
#[command]
pub fn open_in_explorer(path: String) {
  Command::new("explorer")
    .arg(path) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

pub fn pathbuf_to_string(pathbuf: PathBuf) -> MaybeString {
  pathbuf.to_str().map(|st| String::from(st))
}

//测试
#[command]
pub fn test() {
  tracing::info!(target: "frontend_log",message = "test msg" );
}

///这里到时候要做数据库式的配置修改 定位位置后修改那个位置的数据 需要改bug 这里有一步锁死了
#[command]
pub fn update_tidy_folder(path: String) -> Response<Option<bool>> {
  let tidy = PathBuf::from(&path);
  if tidy.exists() {
    let app = &mut crate::app::APP.lock();
    app.conf.tidy_folder = tidy;
    let to_path = app.app_dir.join("conf.toml");
    let a = Conf {
      tidy_folder: PathBuf::from(&path),
      thread: app.conf.thread,
    };
    if let Ok(_) = File::create(&to_path) {
      //写入一个数据
      let c = toml::to_string(&a).unwrap();
      write(&to_path, c).unwrap();
    };

    return Response::new(200, Some(true), "修改成功！");
  } else {
    return Response::new(300, None, "目录不存在");
  }
}

#[command]
pub fn set_focus(label: String, window: Window) {
  match window.get_window(&label) {
    Some(win) => {
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      set_shadow(&win, true).unwrap();
    }
    None => {}
  };

  window
    .emit_all(format!("{}_single", label).as_str(), "")
    .unwrap();
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[command]
#[inline]
pub fn set_style(effect: Effect, label: String, window: Window) {
  match window.get_window(&label) {
    Some(window) => {
      set_shadow(&window, true).unwrap();
      // clear_blur(&window).unwrap();
      // clear_acrylic(&window).unwrap();
      // clear_mica(&window).unwrap();
      // match effect.as_str() {
      //     "blur" => apply_blur(&window, Some((238, 238, 244, 100))).unwrap(),
      //     "acrylic" => apply_acrylic(&window, Some((238, 238, 244, 100))).unwrap(),
      //     "mica" => apply_mica(&window).unwrap(),
      //     _ => (),
      // };
      set_effect(effect, &window);
    }
    None => {}
  };
}

#[command]
pub fn get_local_ip() -> Response<Option<String>> {
  let port = &crate::app::HFSCONFIG.lock().clone().config.port;
  Response::ok(
    Some(format!("{}:{}", local_ipaddress::get().unwrap(), port)),
    "获取成功",
  )
}

pub type MaybeString = Option<String>;
