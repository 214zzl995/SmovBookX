extern crate kuchiki;
extern crate lazy_static;
extern crate reqwest;

use crate::model::seek::SmovSeek;
use crate::serve::file::TidySmov;
use kuchiki::{traits::*, NodeData, NodeRef};
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::path::PathBuf;
use std::{
  fs::File,
  io::{Read, Write},
};
lazy_static! {
  static ref VEC: Vec<u8> = vec![0x18u8, 0x11u8];
  static ref HEADER: HeaderMap = {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".parse().unwrap());
    headers.insert("cookie", "theme=auto; locale=zh; _ym_uid=1643076534203711663; _ym_d=1643076534; _ym_isad=1; over18=1; _jdb_session=u9TcLXlGGbtvm9gGwZYEpinDW9hp8wUpxrV1z88%2Bu6v7DTLIvBn9rUCQBt7O33JtmzPizK4a67uE8E75PJ56YhJQaocudrRi%2B4Ly025mTYamqzR%2FLDSfG5E%2FI32MC05KRngYkB04O%2Blli1jEvGzLLjH7GMDjERWejUQqwWtYVKEOhf2tfP7%2FPk%2BFo8Rg86S1Tai7Zg7Gc1rB0JwUqIMETFc%2BIToWoZ0jNTXWliRGSlhXpvO4Akn%2FuaBu771kG1uiSK0gQPCDTG9hheuFAjjfI0p%2FFV4b4usCkPiZZH3I2vWCM7S%2B4u6uk%2BXs--YVqvN%2Byh43AE6xyR--J5NZMl5Ko12LNJRzk%2Fzbpw%3D%3D".parse().unwrap());
    headers.insert(
      "jdsignature",
      "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
        .parse()
        .unwrap(),
    );
    headers
  };
  static ref MAIN_URL: String = String::from("https://javdb36.com");
}

pub async fn retrieve_smov(format: String, id: i64) -> Result<bool, anyhow::Error> {
  let mut headers = HeaderMap::new();
  headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".parse().unwrap());
  headers.insert("cookie", "theme=auto; locale=zh; _ym_uid=1643076534203711663; _ym_d=1643076534; _ym_isad=1; over18=1; _jdb_session=u9TcLXlGGbtvm9gGwZYEpinDW9hp8wUpxrV1z88%2Bu6v7DTLIvBn9rUCQBt7O33JtmzPizK4a67uE8E75PJ56YhJQaocudrRi%2B4Ly025mTYamqzR%2FLDSfG5E%2FI32MC05KRngYkB04O%2Blli1jEvGzLLjH7GMDjERWejUQqwWtYVKEOhf2tfP7%2FPk%2BFo8Rg86S1Tai7Zg7Gc1rB0JwUqIMETFc%2BIToWoZ0jNTXWliRGSlhXpvO4Akn%2FuaBu771kG1uiSK0gQPCDTG9hheuFAjjfI0p%2FFV4b4usCkPiZZH3I2vWCM7S%2B4u6uk%2BXs--YVqvN%2Byh43AE6xyR--J5NZMl5Ko12LNJRzk%2Fzbpw%3D%3D".parse().unwrap());
  headers.insert(
    "jdsignature",
    "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
      .parse()
      .unwrap(),
  );

  let url = format!("{}/search?q={}&f=all", *MAIN_URL, format);

  let client = reqwest::Client::new();
  let res = client
    .get(url)
    .headers(headers.clone())
    .send()
    .await
    .expect("访问出现错误");

  let text = res.text().await.expect("无法格式化");

  let document = kuchiki::parse_html().one(text);

  let css_selector = "#videos";

  let videos_main = document.select(css_selector).unwrap();

  let mut flag = false;

  //这个循环要改的
  for videos_div in videos_main {
    let videos_main = videos_div.as_node();
    let videos = videos_main.select(".grid-item").unwrap();

    for video in videos {
      let video_item = video.as_node();
      let uid = match video_item.select(".uid").unwrap().next_back() {
        Some(x) => x,
        None => {
          tracing::warn!(target: "frontend_log",message = "未检索到数据");
          return Ok(false);
        } // None
      };

      let name = uid.text_contents();

      let name_f = uid.text_contents().to_uppercase().replace("-", "");

      if name_f == format {
        //在这里直接对数据进行整理 ，能到这里说明数据真实存在
        //smov_file新建一个方法 ，需要对数据更改位置及更新数据库数据，主要目的为修改位置信息,初始化文件夹，需要回传一个path
        //传入的数据应该为 name 和 id ，就能确定 哪条数据和初始化文件夹的名称
        //这里不需要错误回滚机制 因为出现错误时还没有被修改文件的 检索状态 第二次可以直接继续查询
        let s = TidySmov {
          id: &id,
          name: &name,
        };

        let img_to_path = match s.tidy() {
          Ok(n) => n,
          Err(err) => return Err(err),
        };

        let a = video_item.select("a").unwrap().next_back().unwrap();
        let img = video_item.select("img").unwrap().next_back().unwrap();
        let thumbs_url = img.attributes.borrow().get("data-src").unwrap().to_string();
        let att = &a.attributes;
        let href = att.borrow().get("href").unwrap().to_string(); //is_some 是否存在？
        let title = att.borrow().get("title").unwrap().to_string();

        sava_pic(
          &thumbs_url,
          &(format!("thumbs_{}.jpg", name)),
          &img_to_path,
          &client,
        )
        .await
        .expect("保存图片出现错误");

        let url = format!("{}{}", *MAIN_URL, &href);
        let res = client
          .get(&url)
          .headers(headers.clone())
          .send()
          .await
          .expect("出现错误")
          .text()
          .await
          .expect("出现错误");

        let document = kuchiki::parse_html().one(res);

        //这里的错误需要修改 未找到要提前返回
        let video_meta_panel = document
          .select(".video-meta-panel")
          .unwrap()
          .next_back()
          .unwrap();

        let video_meta_panel = video_meta_panel.as_node();

        let smov_img = video_meta_panel
          .select("img")
          .unwrap()
          .next()
          .unwrap()
          .attributes
          .borrow()
          .get("src")
          .unwrap()
          .to_string();

        sava_pic(
          &smov_img,
          &(format!("main_{}.jpg", name)),
          &img_to_path,
          &client,
        )
        .await
        .expect("保存图片出现错误");

        let details = video_meta_panel.select(".panel-block").unwrap();

        let mut smov_seek = SmovSeek {
          id,
          name: name,
          title,
          format: name_f,
          release_time: String::new(),
          duration: 0,
          publishers: String::new(),
          makers: String::new(),
          series: String::from("无系列"),
          directors: String::new(),
          tags: Vec::new(),
          actors: Vec::new(),
        };

        //有两种情况，一种是有系列12个元素 一种没有系列11个元素 按storge检索
        for detail in details {
          let detail_m = detail.as_node();
          let type_flag = match detail_m.select("strong").expect("成功抛出错误").next() {
            Some(e) => e.text_contents(),
            None => continue,
          };

          if type_flag.eq("日期:") {
            let s = detail_m.select(".value").unwrap().next_back().unwrap();
            smov_seek.release_time = s.text_contents();
          } else if type_flag.eq(" 時長:") {
            let s = detail_m
              .select(".value")
              .unwrap()
              .next_back()
              .unwrap()
              .text_contents()
              .replace(" 分鍾", "");
            smov_seek.duration = s.parse::<i32>().unwrap();
          } else if type_flag.eq("導演:") {
            let s = detail_m
              .select(".value")
              .unwrap()
              .next_back()
              .unwrap()
              .text_contents();
            smov_seek.directors = s;
          } else if type_flag.eq("片商:") {
            let s = detail_m
              .select(".value")
              .unwrap()
              .next_back()
              .unwrap()
              .text_contents();
            smov_seek.makers = s;
          } else if type_flag.eq(" 發行:") {
            let s = detail_m
              .select(".value")
              .unwrap()
              .next_back()
              .unwrap()
              .text_contents();
            smov_seek.publishers = s;
          } else if type_flag.eq("系列:") {
            let s = detail_m
              .select(".value")
              .unwrap()
              .next_back()
              .unwrap()
              .text_contents();
            smov_seek.series = s;
          } else if type_flag.eq("類別:") {
            let tags = detail_m.select("a").unwrap();

            for tag in tags {
              let s = tag.as_node().text_contents();
              smov_seek.tags.push(s);
            }
          } else if type_flag.eq("演員:") {
            //有逻辑问题 可能会出现 女演员在后面的情况
            //获取所有链接
            let mut actors = detail_m.select("a").unwrap();
            //获取所有标记
            let actors_flag = detail_m.select(".symbol").unwrap(); //.count();//female

            for symbol in actors_flag {
              let symbol = symbol.as_node();
              if has_class(&symbol, "female") {
                smov_seek
                  .actors
                  .push(actors.next().unwrap().text_contents());
              } else {
                actors.next();
              }
            }
          }
        }

        //保存详情图片
        let screenshots = match document
          .select(".preview-images")
          .expect("获取截图块出现错误")
          .next()
        {
          Some(e) => e,
          None => return Ok(true),
        };

        let screenshots = screenshots
          .as_node()
          .select(".tile-item")
          .expect("获取详情图片快出现错误");

        let mut counter = 1;
        for screenshot in screenshots {
          let screenshot_href = screenshot
            .attributes
            .borrow()
            .get("href")
            .unwrap()
            .to_string();
          // info!("正在保存第{}张图片", &counter);
          sava_pic(
            &screenshot_href,
            &(format!("detail_{}.jpg", counter)),
            &img_to_path.join("detail"),
            &client,
          )
          .await
          .expect("保存图片出现错误");
          counter = counter + 1;
        }

        // SmovSeek::insert_by_path_name(smov_seek).unwrap();
        smov_seek.insert_by_path_name().unwrap();

        flag = true;
      }
    }
  }

  if !flag {
    tracing::warn!(target: "frontend_log",message = "未检索到数据");
    return Ok(false);
  }

  Ok(flag)
}

async fn sava_pic(
  url: &String,
  name: &String,
  path: &PathBuf,
  client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
  let pic_path = path.join(name);

  let mut headers = HeaderMap::new();
  headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".parse().unwrap());
  headers.insert("cookie", "theme=auto; locale=zh; _ym_uid=1643076534203711663; _ym_d=1643076534; _ym_isad=1; over18=1; _jdb_session=u9TcLXlGGbtvm9gGwZYEpinDW9hp8wUpxrV1z88%2Bu6v7DTLIvBn9rUCQBt7O33JtmzPizK4a67uE8E75PJ56YhJQaocudrRi%2B4Ly025mTYamqzR%2FLDSfG5E%2FI32MC05KRngYkB04O%2Blli1jEvGzLLjH7GMDjERWejUQqwWtYVKEOhf2tfP7%2FPk%2BFo8Rg86S1Tai7Zg7Gc1rB0JwUqIMETFc%2BIToWoZ0jNTXWliRGSlhXpvO4Akn%2FuaBu771kG1uiSK0gQPCDTG9hheuFAjjfI0p%2FFV4b4usCkPiZZH3I2vWCM7S%2B4u6uk%2BXs--YVqvN%2Byh43AE6xyR--J5NZMl5Ko12LNJRzk%2Fzbpw%3D%3D".parse().unwrap());
  headers.insert(
    "jdsignature",
    "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
      .parse()
      .unwrap(),
  );

  let msg = format!(
    "保存图片url:{} => path:{}",
    url,
    path.as_os_str().to_str().unwrap_or_else(|| "none")
  );

  tracing::info!(target: "frontend_log",message = msg.as_str());

  let res = client
    .get(url)
    .headers(headers)
    .send()
    .await?
    .bytes()
    .await?;

  let mut file = match File::create(&pic_path) {
    Err(why) => panic!("couldn't create {}", why),
    Ok(file) => file,
  };

  let content = res.bytes();
  let data: std::result::Result<Vec<_>, _> = content.collect();
  file.write_all(&data.unwrap())?;

  Ok(())
}

pub fn has_class(el: &NodeRef, class: &str) -> bool {
  let data = match el.data() {
    NodeData::Element(data) => data,
    _ => return false,
  };

  let attributes = data.attributes.borrow();

  if let Some(class_attr) = attributes.get("class") {
    class_attr.split_whitespace().any(|piece| piece == class)
  } else {
    false
  }
}
