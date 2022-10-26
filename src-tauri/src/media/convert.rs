use std::fs::create_dir_all;

use anyhow::anyhow;
use anyhow::Error;
use gstreamer::{
  prelude::Cast,
  traits::{ElementExt, GstObjectExt},
};

use crate::model::smov::Smov;
use crate::task::pool::TaskStatus;

impl Smov {
  pub fn to_hls(self: &Self, task: &crate::task::pool::Task<'_>) -> Result<(), Error> {
    task.emit_status(
      TaskStatus::Running,
      format!("开始执行转码，转码名称：{}", self.name).as_str(),
    );
    let path = crate::app::APP.lock().clone().conf.tidy_folder;
    let media_folder = path.join(self.realname.clone());

    let media = media_folder
      .join(format!("{}.{}", self.realname, self.extension))
      .clone();

    let ts_path = media_folder.join("hls").clone();

    create_dir_all(&ts_path).expect("创建HLS文件夹错误");

    let commond = format!("filesrc location={} name=file ! qtdemux  name=demux  ! queue ! progressreport update-freq=1 silent=true ! h264parse  ! mpegtsmux name=mux ! hlssink  playlist-length=0 target-duration=5 max-files=99999999 playlist-location={}/playlist.m3u8 location={}/fragment%03d.ts name=sink demux. ! queue ! aacparse ! mux. " , media.display(),ts_path.display(),ts_path.display()).replace("\\", "/");

    gstreamer::init().unwrap();

    //可以配置GST_PLUGIN_PATH
    #[cfg(debug_assertions)]
    {}

    #[cfg(not(debug_assertions))]
    {
      use std::path::Path;

      let mut path = Path::new("./gst-plugins");
      if !path.exists() {
        path = Path::new("./gst-plugins");
      }

      gstreamer::Registry::get().scan_path(path);
    }

    let pipeline = match gstreamer::parse_launch(&commond) {
      Ok(ele) => ele.downcast::<gstreamer::Pipeline>().unwrap(),
      Err(err) => {
        tracing::error!("{}", err.message());
        return Err(anyhow!("执行转码错误:{}", err));
      }
    };

    pipeline.set_state(gstreamer::State::Playing).unwrap();

    let bus = pipeline
      .bus()
      .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
      use gstreamer::MessageView;
      match msg.view() {
        MessageView::Eos(..) => {
          println!("EOS");
          break;
        }
        MessageView::Error(err) => {
          pipeline.set_state(gstreamer::State::Null).unwrap();
          let err_msg = format!(
            "转码错误 {}: {} ({})",
            msg
              .src()
              .map(|s| String::from(s.path_string()))
              .unwrap_or_else(|| "None".into()),
            err.error(),
            err.debug().unwrap_or_else(|| "".into()),
          );
          return Err(anyhow!(err_msg));
        }
        MessageView::Element(ele) => {
          let structure = ele.structure().unwrap();
          let percent = structure.get::<i32>("percent").unwrap();
          println!("{}", percent);
        }
        _ => {}
      }
    }

    pipeline.set_state(gstreamer::State::Null).unwrap();

    task.emit_status(TaskStatus::Success, "转码结束,转码状态：成功");

    Ok(())
  }
}
