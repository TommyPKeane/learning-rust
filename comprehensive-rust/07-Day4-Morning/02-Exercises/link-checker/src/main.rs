use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    let mut valid_urls = Vec::new();
    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => valid_urls.push(url),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }

    Ok(valid_urls)
}

fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();
    let response = get(start_url).unwrap();
    match extract_links(response) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
