use anyhow::anyhow;
use anyhow::Result;
use std::io;
use std::sync::OnceLock;

use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Proxy;
use scraper::{ElementRef, Selector};

static CLI: OnceLock<Client> = OnceLock::new();

pub fn http() -> &'static Client {
    CLI.get_or_init(|| {
        ClientBuilder::default()
            .proxy(Proxy::http("socks5://127.0.0.1:7890").unwrap())
            .build()
            .expect("Init reqwest client failed")
    })
}

pub fn wait_user_input_a_number(min: Option<i32>, max: Option<i32>) -> i32 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input an number");
                continue;
            }
        };
        if min.is_some() && min.unwrap() > number {
            println!("Please input an number must great than {}", min.unwrap());
            continue;
        }
        if max.is_some() && max.unwrap() < number {
            println!("Please input an number must less than {}", max.unwrap());
            continue;
        }
        return number;
    }
}

fn find_el_by_selectors<'a>(
    el: ElementRef<'a>,
    selectors: Vec<&str>,
) -> Result<Vec<ElementRef<'a>>> {
    let mut inner = el;

    let last_index = selectors.len() - 1;
    for (index, selector) in selectors.iter().enumerate() {
        let selector = Selector::parse(selector);
        if selector.is_err() {
            return Err(anyhow!("The selectors is err!"));
        }
        if index == last_index {
            return Ok(inner.select(&selector.unwrap()).collect());
        } else {
            inner = inner
                .select(&selector.unwrap())
                .next()
                .ok_or(anyhow!("There has nothing what you want to find"))?;
        }
    }
    Err(anyhow!("There has nothing what you want to find"))
}

fn find_first_el_by_selectors<'a>(el: &'a ElementRef, selectors: &str) -> Result<ElementRef<'a>> {
    let selector = Selector::parse(selectors);
    if selector.is_err() {
        return Err(anyhow!("The selectors is err!"));
    }

    el.select(&selector.unwrap())
        .next()
        .ok_or(anyhow!("There has nothing what you want to find."))
}

fn extract_text(el: &ElementRef) -> String {
    el.text().collect::<String>().trim().to_string()
}

fn extract_text_from_selectors(el: &ElementRef, selectors: &str) -> Result<String> {
    Ok(extract_text(&find_first_el_by_selectors(el, selectors)?))
}
