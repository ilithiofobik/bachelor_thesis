extern crate reqwest;
extern crate scraper;

use normalize_url::normalizer;
use scraper::{Html, Selector};
use std::collections::HashSet;

/// A scraper for HTML documents finding links to other pages.
/// It contains an optional word that must be contained within the url address of found pages.
pub struct Scraper {
    client: reqwest::blocking::Client,
    must_contain: Option<String>,
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
            client: reqwest::blocking::Client::new(),
            must_contain,
        }
    }

    /// Scrapes the given url for links to other pages while normalizing their urls.
    /// # Examples
    /// ```
    /// use bipartite::scraper::Scraper;
    /// let scraper = Scraper::new(None);
    /// let links = scraper.scrape("https://pwr.edu.pl/");
    /// assert!(links.len() > 0);
    /// ```
    pub fn scrape(&self, url: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let resp = self.client.get(url).send();
        if let Ok(resp) = resp {
            let body = resp.text().unwrap();
            let document = Html::parse_document(&body);
            let selector = Selector::parse("a").unwrap();
            match self.must_contain {
                Some(ref word) => {
                    for link in document.select(&selector) {
                        let href = link.value().attr("href").unwrap_or_default();
                        if href.starts_with("http") && href.contains(word) {
                            if let Ok(normalizer) = normalizer::UrlNormalizer::new(href) {
                                if let Ok(normalized) = normalizer.normalize(None) {
                                    result.insert(normalized.to_owned());
                                }
                            }
                        }
                    }
                }
                None => {
                    for link in document.select(&selector) {
                        let href = link.value().attr("href").unwrap_or_default();
                        if href.starts_with("http") {
                            if let Ok(normalizer) = normalizer::UrlNormalizer::new(href){
                                if let Ok(normalized) = normalizer.normalize(None) {
                                    result.insert(normalized.to_owned());
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }
}
