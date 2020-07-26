use super::ports::Port;

use reqwest::get;

use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
  foreign_links {
      ReqError(reqwest::Error);
      IoError(std::io::Error);
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
  pub title: String,
  pub port: Port,
}

pub async fn head_port_local(port: Port) -> Option<Location> {
  let response = get(&port.to_local_url()).await.unwrap();

  if response.status().is_success() {
    let body: String = response.text().await.unwrap();
    // let is_nextjs: bool = body.contains("_next");
    let document = Document::from(body.as_str());
    let node = document.find(Name("title")).next();

    if node.is_some() {
      let title: String = node.unwrap().text();
      Some(Location {
        port: port,
        title: title,
      })
    } else {
      Some(Location {
        port: port,
        title: String::from("Untiteled"),
      })
    }
  } else {
    println!("Fail: {}", response.status());
    None
  }
}
