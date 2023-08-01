use anyhow::Result;
use reqwest::blocking::ClientBuilder;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::find::Book;

/// 无名图书
///
/// [查看](https://www.book123.info)
struct Nameless {
    search_url: String,
    download_url: String,
    count: u8,
    page: u8,
    key: String,
}

impl Nameless {
    fn new(keyword: &str) -> Self {
        Self {
            search_url: String::from("https://www.book123.info/api/simple_search"),
            download_url: String::from("https://static2.file123.info"),
            count: 5,
            page: 1,
            key: keyword.to_string(),
        }
    }
    fn get_query_url(&self) -> Url {
        let mut url = Url::parse(&self.search_url).unwrap();
        url.set_query(Some(&format!("{}={}&{}={}&{}={}", "count", &self.count, "page", &self.page, "key", &self.key)));
        url
    }
}

pub fn find(keyword: &str) -> Result<Vec<Book>> {
    let nameless = Nameless::new(keyword);

    Ok(ClientBuilder::default().build()?
        .get(nameless.get_query_url()).send()?
        .json::<SimpleSearchRes>()?
        .books.into_iter()
        .filter(|b| !b.download_url.is_empty())
        .map(Into::<Book>::into)
        .map(|mut b| {
            b.download = format!("{}{}", nameless.download_url, b.download);
            b
        }).collect())
}

impl From<NamelessBook> for Book {
    fn from(value: NamelessBook) -> Self {
        Self {
            title: value.title,
            author: value.author.as_ref().map_or("".to_string(), |a| a.to_string()),
            isbn: value.isbn,
            tag: "".to_string(),
            score: "".to_string(),
            size: value.size,
            download: value.download_url,
            file_type: value.file_type,
            source: "无名图书".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase", default)]
struct NamelessBook {
    title: String,
    author: Option<String>,
    isbn: String,
    pub_date: String,
    db_img: String,
    summary: String,
    size: String,
    file_type: String,
    download_url: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase", default)]
struct SimpleSearchRes {
    books: Vec<NamelessBook>,
    key: String,
}