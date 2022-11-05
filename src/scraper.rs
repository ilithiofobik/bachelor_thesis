extern crate reqwest;
extern crate scraper;

use normalize_url::normalizer;
use scraper::{Html, Selector};
use std::collections::HashSet;

/// A scraper for HTML documents finding links to other pages.
/// It contains a set of stop words and words that must be contained in the link.
pub struct Scraper {
    client: reqwest::blocking::Client,
    must_contain: Vec<String>,
    stop_words: Vec<String>,
}

impl Scraper {
    /// Creates a new Scraper with the given word that must be contained within the url address of found pages.
    /// If the word is None, then no word is required.
    /// It also takes a vector of stop words that are not allowed to be contained within the url address of found pages.
    /// # Examples
    /// ```
    /// use bipartite::scraper::Scraper;
    /// let scraper = Scraper::new(vec![], vec![]);
    /// ```
    pub fn new(must_contain: Vec<String>, stop_words: Vec<String>) -> Scraper {
        Scraper {
            client: reqwest::blocking::Client::new(),
            must_contain,
            stop_words,
        }
    }

    /// Scrapes the given url for links to other pages while normalizing their urls.
    /// If the connection to the url fails, then an empty HashSet is returned.
    /// # Examples
    /// ```
    /// use bipartite::scraper::Scraper;
    /// let scraper = Scraper::new(vec![], vec![]);
    /// let links = scraper.scrape("https://pwr.edu.pl/");
    /// assert!(links.len() > 0);
    /// for link in &links {
    ///     assert!(link.starts_with("https://"));
    /// }
    /// ```
    pub fn scrape(&self, url: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let resp = self.client.get(url).send();
        if let Ok(resp) = resp {
            if let Ok(body) = resp.text() {
                let document = Html::parse_document(&body);
                if let Ok(selector) = Selector::parse("a") {
                    for link in document.select(&selector) {
                        let href = link.value().attr("href").unwrap_or_default();
                        if href.starts_with("http") 
                        && self.must_contain.iter().all(|word| href.contains(word)) // must contain
                        && self.stop_words.iter().all(|word| !href.contains(word)) { // must not contain
                            if let Ok(normalizer) = normalizer::UrlNormalizer::new(href) {
                                if let Ok(normalized) = normalizer.normalize(None) {
                                    let mut normalized = normalized.to_owned();
                                    if normalized.chars().nth(4) == Some(':') {
                                        normalized.insert(4, 's');
                                    }
                                    result.insert(normalized);
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
