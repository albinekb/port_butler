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
  title: String,
  port: Port,
}

pub async fn head_port_local(port: Port) -> Option<Location> {
  // let timeout = Duration::new(5, 0);
  // let client = ClientBuilder::new().timeout(timeout).build()?;
  // let response = client.get(&port.to_local_url()).send().await?;
  // let mut resp = reqwest::get(&port.to_local_url()).unwrap();
  let response = get(&port.to_local_url()).await.unwrap();

  if response.status().is_success() {
    println!("Ok: {}", response.status());
    let body: String = response.text().await.unwrap();
    let is_nextjs: bool = body.contains("_next");

    let title: String = Document::from(body.as_str())
      .find(Name("title"))
      .next()
      .unwrap()
      .text();

    println!("Title {:?}", title);
    println!("is nextjs {:?}", is_nextjs);
    return Some(Location {
      port: port,
      title: title,
    });
  } else {
    println!("Fail: {}", response.status());
  }

  None
}
