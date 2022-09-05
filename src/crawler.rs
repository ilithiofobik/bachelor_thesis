use super::graphs::Graph;
use super::scraper::Scraper;

pub struct Crawler {
    input: String,
    max_depth: usize,
    must_contain: Option<String>
}

impl Crawler {
    /// Create a new parser.
    /// ```
    /// use bipartite::crawler::Crawler;
    /// let parser = Crawler::new("<html><body><p>Hello</p></body></html>".to_owned(), 1, None);
    /// ```
    pub fn new(input: String, max_depth: usize, must_contain: Option<String>) -> Crawler {
        Crawler {
            input,
            max_depth,
            must_contain
        }
    }

    /// Crawl the web based on given url and max_depth.
    /// ```
    /// use bipartite::crawler::Crawler;
    /// let crawler = Crawler::new("https://pwr.edu.pl/".to_owned(), 1, None);
    /// let links = crawler.crawl();
    /// assert!(true);
    /// ```
    pub fn crawl(&self) -> Graph {
        let num_of_threads = num_cpus::get_physical();
        
        let scrapers = {
            let mut scrapers = Vec::with_capacity(num_of_threads);
            (0..num_of_threads).into_iter().for_each(|_| scrapers.push(Scraper::new(self.must_contain.clone())));
            scrapers
        };      

        Graph::empty()
    }
}
