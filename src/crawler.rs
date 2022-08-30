
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
    /// let links = Crawler::crawl();
    /// assert_eq!(links, 4);
    /// ```
    pub fn crawl() -> usize {
        let num_of_threads = num_cpus::get_physical();
        num_of_threads
    }
}
