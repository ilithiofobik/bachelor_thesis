use core::num;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock}; //mpsc to be added

use super::graphs::Graph;
use super::scraper::Scraper;

pub struct Crawler {
    root: String,
    max_depth: usize,
    must_contain: Option<String>
}

impl Crawler {
    /// Create a new parser.
    /// ```
    /// use bipartite::crawler::Crawler;
    /// let parser = Crawler::new("<html><body><p>Hello</p></body></html>".to_owned(), 1, None);
    /// ```
    pub fn new(root: String, max_depth: usize, must_contain: Option<String>) -> Crawler {
        Crawler {
            root,
            max_depth,
            must_contain
        }
    }

    /// Crawl the web based on given url and max_depth.
    /// ```
    /// use bipartite::crawler::Crawler;
    /// let crawler = Crawler::new("https://pwr.edu.pl/".to_owned(), 3, Some("pwr.edu".to_owned()));
    /// let links = crawler.crawl();
    /// assert_eq!(links.idx_to_name(0).unwrap(), "https://pwr.edu.pl/");
    /// ```
    pub fn crawl(&self) -> Graph {
        let num_of_threads = num_cpus::get_physical();
        let max_depth = Arc::new(self.max_depth); // to share between threads and not to be changed
        let graph = {
            let graph = Graph::from_names(vec![self.root.to_owned()]);
            Arc::new(RwLock::new(graph))
        };
        let nodes_to_scan = {
            let mut queue = VecDeque::new();
            queue.push_back((0,0)); // (depth, node_id)
            Arc::new(RwLock::new(queue)) 
        };  
        let scrapers = {
            let mut scrapers = Vec::with_capacity(num_of_threads);
            (0..num_of_threads).into_iter().for_each(|_| scrapers.push(Arc::new(Scraper::new(self.must_contain.clone()))));
            scrapers
        }; // scrapers are used but not changed     

        let curr_num_of_threads = std::cmp::min(num_of_threads, nodes_to_scan.read().unwrap().len());
        let mut threads = Vec::with_capacity(curr_num_of_threads);
        for t in 0..curr_num_of_threads {
            let scraper = Arc::clone(&scrapers[t]);
            let graph = Arc::clone(&graph);
            let nodes_to_scan = Arc::clone(&nodes_to_scan);
            let max_depth = Arc::clone(&max_depth);
            threads.push(std::thread::spawn(move || {
                let graph = graph.read().unwrap();
                let node_to_scan = nodes_to_scan.read().unwrap().get(t);
            }));
        }

        let graph_r = graph.read().unwrap();
        graph_r.clone()
    }
}
