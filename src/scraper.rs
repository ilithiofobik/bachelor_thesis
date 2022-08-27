extern crate reqwest;
extern crate scraper;

use std::collections::HashSet;
use scraper::{Html, Selector};

/// A scraper for HTML documents finding links to other pages. 
/// It contains an optional word that must be contained within the url address of found pages.
pub struct Scraper {
    client : reqwest::Client,
    must_contain : Option<String>
}

impl Scraper {
    /// Creates a new Scraper with the given word that must be contained within the url address of found pages.
    /// If the word is None, then no word is required.
    /// # Examples
    /// ```
    /// use bipartite::scraper::Scraper;
    /// let scraper = Scraper::new(None);
    /// ```
    pub fn new(must_contain: Option<String>) -> Scraper {
        Scraper {
            client : reqwest::Client::new(),
            must_contain
        }
    }

    /// Scrapes the given url for links to other pages.
    /// # Examples
    /// ```
    /// use bipartite::scraper::Scraper;
    /// let scraper = Scraper::new(None);
    /// let links = tokio_test::block_on(scraper.scrape("https://www.rust-lang.org/en-US/"));
    /// assert!(links.len() > 0);
    /// ```
    pub async fn scrape(&self, url: &str) -> Vec<String> {
        let mut result = HashSet::new();
        let resp = self.client.get(url).send().await;
        if let Ok(resp) = resp {
            let body = resp.text().await.unwrap();
            let document = Html::parse_document(&body);
            let selector = Selector::parse("a").unwrap();
            match self.must_contain {
                Some(ref word) => {
                    for link in document.select(&selector) {
                        let value = link.value().attr("href").unwrap();
                        if value.starts_with("http") && value.contains(word) {
                            result.insert(value.to_string());
                        }
                    }
                }
                None => {
                    for link in document.select(&selector) {
                        let value = link.value().attr("href").unwrap();
                        if value.starts_with("http") {
                            result.insert(value.to_string());
                        }
                    }
                }
            }               
        }
        result.into_iter().collect()
    }
}
