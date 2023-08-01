use std::fs::File;
use std::io::{Read, Write};

use prettytable::{format, row, Table};
use reqwest::blocking::ClientBuilder;

use crate::util;

mod nameless;

pub fn find(keyword: &str) {
    let books = nameless::find(keyword);
    if books.is_err() {
        println!("There has an error: {:?}", books.err());
        return;
    }
    let books = books.unwrap();

    if books.is_empty() {
        println!("Sorry! we found nothing");
        return;
    }
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row![
        "Number", "Title", "Author", "Type", "Size", "ISBN", "Source"
    ]);
    for (i, book) in books.iter().enumerate() {
        table.add_row(row![
            i + 1,
            book.title,
            book.author,
            book.file_type,
            book.size,
            book.isbn,
            book.source
        ]);
    }
    table.printstd();
    println!("Please input a number of the book that you want to download");
    let number = util::wait_user_input_a_number(Some(1), Some(books.len() as i32));
    let book = books.get(number as usize).unwrap();

    let download = &book.download;
    println!("{}", download);

    // let proxy = Proxy::http("socks5://127.0.0.1:7890").unwrap();
    let cli = ClientBuilder::default().build().unwrap();

    let mut response = cli.get(download).send().unwrap();

    if response.status().is_success() {
        let mut file = File::create(format!("{}.{}", book.title, book.file_type))
            .expect("Failed to create file");

        let mut buffer = [0; 1024 * 1024];

        loop {
            match response.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    file.write_all(&buffer[..n])
                        .expect("Failed to write to file");
                }
                Err(e) => {
                    println!("Error occurred while downloading file: {}", e);
                    break;
                }
            }
        }
        println!("File downloaded successfully!");
    } else {
        println!(
            "Failed to download file. Response status: {:?}",
            response.status()
        );
    }
}

#[derive(Default, Debug)]
pub struct Book {
    title: String,
    author: String,
    isbn: String,
    tag: String,
    score: String,
    size: String,
    download: String,
    file_type: String,
    source: String,
}
