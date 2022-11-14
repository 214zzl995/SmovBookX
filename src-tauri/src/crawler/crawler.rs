use std::path::PathBuf;

use crate::{
  crawler::{
    error::CrawlerErr,
    network::{get_temp_sync, sava_pic_sync},
  },
  model::{
    file_seek::{RetrievingSmov, SmovFileSeek},
    seek::SmovSeek,
  },
  response::response::Response,
  serve::file::TidySmov,
  task::pool::TaskStatus,
  util::smov_format::SmovName,
};
use anyhow::Result;

///考虑在爬虫实现热更新 模板引擎的错误尝试 模板引擎可能并不适合爬虫 有没有直接从云端获取结构的办法 哎 到时候再解决吧
use scraper::{ElementRef, Selector};
use tauri::command;

use super::client::MAIN_URL;

#[command]
pub async fn smov_crawler(retrieving_smov: RetrievingSmov) -> Response<Option<i32>> {
  let format = SmovName::format_smov_name(&retrieving_smov.seek_name);
  match smov_crawler_program(format, retrieving_smov.smov_id).await {
    Ok(_) => {
      SmovFileSeek::change_status(retrieving_smov.id, 1).expect("出现了一个错误");
      Response::new(200, Some(1), "success")
    }
    Err(err) => {
      SmovFileSeek::change_status(retrieving_smov.id, 2).expect("出现了一个错误");
      Response::new(404, Some(1), &err.to_string())
    }
  }
}

async fn smov_crawler_program(format: String, id: i64) -> Result<()> {
  let url = format!("{}/search?q={}&f=all", *MAIN_URL, format);

  let fragment = match get_temp_sync(&url) {
    Ok(html) => html,
    Err(err) => return Err(err),
  };

  let selector = Selector::parse(".movie-list").unwrap();

  let movie_list = match fragment.select(&selector).next() {
    Some(list) => list,
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let movie_list_item_select = Selector::parse(".item").unwrap();

  let name_selector = Selector::parse("strong").unwrap();

  let smov_item = movie_list
    .select(&movie_list_item_select)
    .filter(|&movie_item| {
      if let Some(name) = movie_item.select(&name_selector).next() {
        SmovName::format_smov_name(&name.inner_html()).eq(&format)
      } else {
        false
      }
    })
    .collect::<Vec<_>>();

  let smov_item = match smov_item.first() {
    Some(item) => item,
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let name = match smov_item.select(&name_selector).next() {
    Some(name) => name.inner_html(),
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let s = TidySmov {
    id: &id,
    name: &name,
  };

  let img_to_path = match s.tidy() {
    Ok(n) => n,
    Err(err) => return Err(err),
  };

  let item_path = smov_item
    .select(&Selector::parse("a").unwrap())
    .next()
    .unwrap();
  let img = smov_item
    .select(&Selector::parse("img").unwrap())
    .next()
    .unwrap();

  let title = item_path
    .value()
    .attr("title")
    .unwrap_or_else(|| "")
    .to_string();

  let item_url = item_path.value().attr("href").unwrap_or_else(|| "");

  let thumbs_url = img.value().attr("src").unwrap_or_else(|| "");

  tracing::info!("title:{}", title);

  match sava_pic_sync(
    thumbs_url.to_string(),
    format!("thumbs_{}.jpg", &name),
    PathBuf::from(&img_to_path),
  ) {
    Err(err) => return Err(err),
    _ => {}
  };

  let url = format!("{}{}", *MAIN_URL, item_url.to_string());

  tracing::info!("url:{}", url);

  let fragment = match get_temp_sync(&url) {
    Ok(html) => html,
    Err(err) => return Err(err),
  };

  let mut smov_seek = SmovSeek {
    id,
    name: String::from(&name),
    title,
    format,
    release_time: String::new(),
    duration: 0,
    publishers: String::new(),
    makers: String::new(),
    series: String::from("无系列"),
    directors: String::new(),
    tags: Vec::new(),
    actors: Vec::new(),
  };

  let selector = Selector::parse(".video-meta-panel").unwrap();

  let mut video_meta_panel = fragment.select(&selector);

  let video_meta_panel = match video_meta_panel.next() {
    Some(el) => el,
    None => {
      return Err(anyhow::Error::new(CrawlerErr::ItemNotFound));
    }
  };

  match sava_pic_sync(
    thumbs_url.to_string(),
    format!("main_{}.jpg", &name),
    PathBuf::from(&img_to_path),
  ) {
    Err(err) => return Err(err),
    _ => {}
  };

  let selector = Selector::parse(".panel-block").unwrap();

  let strong_selector = Selector::parse("strong").unwrap();

  let value_selector = Selector::parse(".value").unwrap();

  for detail in video_meta_panel.select(&selector) {
    if let Some(strong_type_el) = detail.select(&strong_selector).next() {
      match strong_type_el.inner_html().as_str() {
        "時長:" => {
          smov_seek.duration = get_value_unique(detail, &value_selector)
            .replace(" 分鍾", "")
            .parse::<i32>()
            .unwrap()
        }
        "日期:" => smov_seek.release_time = get_value_unique(detail, &value_selector),
        "導演:" => smov_seek.directors = get_value_unique(detail, &value_selector),
        "片商:" => smov_seek.makers = get_value_unique(detail, &value_selector),
        "發行:" => smov_seek.publishers = get_value_unique(detail, &value_selector),
        "系列:" => smov_seek.series = get_value_unique(detail, &value_selector),
        "類別:" => smov_seek.tags = get_value(detail, &value_selector),
        "演員:" => smov_seek.actors = get_value_actors(detail, &value_selector),
        _ => {}
      };
    };
  }

  let selector = Selector::parse(".preview-images").unwrap();

  let detail_images = fragment.select(&selector).next().unwrap();

  let selector = Selector::parse(".tile-item").unwrap();

  let mut counter = 1;

  for deimg in detail_images.select(&selector) {
    let path = deimg.value().attr("href").unwrap().to_string();
    match sava_pic_sync(
      path,
      format!("detail_{}.jpg", counter),
      PathBuf::from(&img_to_path).join("detail"),
    ) {
      Err(err) => return Err(err),
      _ => {}
    };
    counter += 1;
  }

  smov_seek.insert_by_path_name().unwrap();

  Ok(())
}

//20221111 保留爬虫爬取的网址
pub fn smov_crawler_program_pool(
  format: String,
  id: i64,
  task: &crate::task::pool::Task<'_>,
) -> Result<()> {
  task.emit_status(
    TaskStatus::Running,
    format!("开始爬取 爬取名称：{}", format).as_str(),
  );

  let url = format!("{}/search?q={}&f=all", *MAIN_URL, format);

  let fragment = match get_temp_sync(&url) {
    Ok(html) => html,
    Err(err) => return Err(err),
  };

  let selector = Selector::parse(".movie-list").unwrap();

  let movie_list = match fragment.select(&selector).next() {
    Some(list) => list,
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let movie_list_item_select = Selector::parse(".item").unwrap();

  let name_selector = Selector::parse("strong").unwrap();

  let smov_item = movie_list
    .select(&movie_list_item_select)
    .filter(|&movie_item| {
      if let Some(name) = movie_item.select(&name_selector).next() {
        SmovName::format_smov_name(&name.inner_html()).eq(&format)
      } else {
        false
      }
    })
    .collect::<Vec<_>>();

  let smov_item = match smov_item.first() {
    Some(item) => item,
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let name = match smov_item.select(&name_selector).next() {
    Some(name) => name.inner_html(),
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let tidy_smov = TidySmov {
    id: &id,
    name: &name,
  };

  let count = tidy_smov.get_name_count().unwrap();

  //判断当前的是否已经存在 当存在时 只调用文件处理 需要分隔当前的程序 这个程序步骤全挤在一起了
  if count > 0 {
    tidy_smov.tidy_when_exist(count)
  } else {
    let img_to_path = match tidy_smov.tidy() {
      Ok(n) => n,
      Err(err) => return Err(err),
    };

    let item_path = smov_item
      .select(&Selector::parse("a").unwrap())
      .next()
      .unwrap();
    let img = smov_item
      .select(&Selector::parse("img").unwrap())
      .next()
      .unwrap();

    let title = item_path
      .value()
      .attr("title")
      .unwrap_or_else(|| "")
      .to_string();

    let item_url = item_path.value().attr("href").unwrap_or_else(|| "");

    let thumbs_url = img.value().attr("src").unwrap_or_else(|| "");

    tracing::info!("title:{}", title);

    match sava_pic_sync(
      thumbs_url.to_string(),
      format!("thumbs_{}.jpg", &name),
      PathBuf::from(&img_to_path),
    ) {
      Err(err) => return Err(err),
      _ => {}
    };

    let url = format!("{}{}", *MAIN_URL, item_url.to_string());

    tracing::info!("url:{}", url);

    let fragment = match get_temp_sync(&url) {
      Ok(html) => html,
      Err(err) => return Err(err),
    };

    let mut smov_seek = SmovSeek {
      id,
      name: String::from(&name),
      title,
      format,
      release_time: String::new(),
      duration: 0,
      publishers: String::new(),
      makers: String::new(),
      series: String::from("无系列"),
      directors: String::new(),
      tags: Vec::new(),
      actors: Vec::new(),
    };

    let selector = Selector::parse(".video-meta-panel").unwrap();

    let mut video_meta_panel = fragment.select(&selector);

    let video_meta_panel = match video_meta_panel.next() {
      Some(el) => el,
      None => {
        return Err(anyhow::Error::new(CrawlerErr::ItemNotFound));
      }
    };

    match sava_pic_sync(
      thumbs_url.to_string(),
      format!("main_{}.jpg", &name),
      PathBuf::from(&img_to_path),
    ) {
      Err(err) => return Err(err),
      _ => {}
    };

    let selector = Selector::parse(".panel-block").unwrap();

    let strong_selector = Selector::parse("strong").unwrap();

    let value_selector = Selector::parse(".value").unwrap();

    for detail in video_meta_panel.select(&selector) {
      if let Some(strong_type_el) = detail.select(&strong_selector).next() {
        match strong_type_el.inner_html().as_str() {
          "時長:" => {
            smov_seek.duration = get_value_unique(detail, &value_selector)
              .replace(" 分鍾", "")
              .parse::<i32>()
              .unwrap()
          }
          "日期:" => smov_seek.release_time = get_value_unique(detail, &value_selector),
          "導演:" => smov_seek.directors = get_value_unique(detail, &value_selector),
          "片商:" => smov_seek.makers = get_value_unique(detail, &value_selector),
          "發行:" => smov_seek.publishers = get_value_unique(detail, &value_selector),
          "系列:" => smov_seek.series = get_value_unique(detail, &value_selector),
          "類別:" => smov_seek.tags = get_value(detail, &value_selector),
          "演員:" => smov_seek.actors = get_value_actors(detail, &value_selector),
          _ => {}
        };
      };
    }

    let selector = Selector::parse(".preview-images").unwrap();

    let detail_images = fragment.select(&selector).next().unwrap();

    let selector = Selector::parse(".tile-item").unwrap();

    let mut counter = 1;

    for deimg in detail_images.select(&selector) {
      let path = deimg.value().attr("href").unwrap().to_string();
      match sava_pic_sync(
        path,
        format!("detail_{}.jpg", counter),
        PathBuf::from(&img_to_path).join("detail"),
      ) {
        Err(err) => return Err(err),
        _ => {}
      };
      counter += 1;
    }

    smov_seek.insert_by_path_name().unwrap();

    task.emit_status(TaskStatus::Success, "爬取结束,爬取状态：成功");

    Ok(())
  }
}

///用于获取value的部分
fn get_value_unique(el: ElementRef, selector: &Selector) -> String {
  let text = el.select(selector).next().unwrap().text();
  let mut values = "".to_string();
  for value in text {
    values = format!("{}{}", values, value);
  }

  values
}

fn get_value(el: ElementRef, selector: &Selector) -> Vec<String> {
  let value = el
    .select(selector)
    .next()
    .unwrap()
    .text()
    .map(|x| x.to_string())
    .filter(|x| !x.eq(&",\u{a0}".to_string())) //去除空格部分
    .collect::<Vec<_>>();

  value
}

fn get_value_actors(el: ElementRef, selector: &Selector) -> Vec<String> {
  let el = el.select(selector).next().unwrap();
  //先获取演员部分
  let selectors = Selector::parse("a").unwrap();

  let mut actors = el.select(&selectors);

  //再获取标记部分
  let selectors = Selector::parse("strong").unwrap();
  let flags = el.select(&selectors);

  let value = flags
    .map(|x| {
      //判断字符是否为 ♀
      if x.inner_html().eq("♀") {
        actors.next().unwrap().inner_html()
      } else {
        actors.next();
        "".to_string()
      }
    })
    .filter(|x| !x.eq(&"".to_string()))
    .collect::<Vec<_>>();

  value
}
