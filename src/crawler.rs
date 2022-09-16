use std::collections::VecDeque;
use std::sync::{Arc, RwLock, mpsc};

use super::graphs::Graph;
use super::scraper::Scraper;

enum Index {
    StrIndex(String),
    NumIndex(usize),
}

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
        
        let mut curr_num_of_threads = 0;
        while curr_num_of_threads > 0 {
            let mut threads = Vec::with_capacity(curr_num_of_threads);
            let (tx, rx) = mpsc::channel();

            for t in 0..curr_num_of_threads {
                let scraper = Arc::clone(&scrapers[t]);
                let graph = Arc::clone(&graph);
                let nodes_to_scan_clone = Arc::clone(&nodes_to_scan);
                let max_depth = Arc::clone(&max_depth);
                let tx = tx.clone();

                threads.push(std::thread::spawn(move || {
                    let graph = graph.read().unwrap();
                    let (depth, node_id) = *nodes_to_scan_clone.read().unwrap().get(curr_num_of_threads).unwrap();
                    let root_node_name = graph.idx_to_name(node_id).unwrap();
                    let links = scraper.scrape(&root_node_name);
                    let links = 
                        if depth == *max_depth {
                            links.into_iter().filter(|link| graph.contains_vertix(link)).collect()
                        } else {
                            links
                        };
                    let links = links.into_iter().map(|link| {
                        let link = link;
                        let index = graph.name_to_idx(&link);
                        match index {
                            Ok(idx) => Index::NumIndex(idx),
                            Err(_) => Index::StrIndex(link)
                        }
                    }).collect::<Vec<Index>>();

                    tx.send((root_node_name, depth, links)).unwrap();
                }));
            }

            for _ in 0..curr_num_of_threads {
                let _new_indiviual = rx.recv().unwrap();
               // population.push(new_indiviual);
            }

            for thread in threads {
                thread.join().expect("The thread creating or execution failed!");
            }
            // deleting scanned nodes
            let mut nodes_to_scan_write = nodes_to_scan.write().unwrap();
            (0..curr_num_of_threads).into_iter().for_each(|_| {
                nodes_to_scan_write.pop_front();
            });
            curr_num_of_threads = std::cmp::min(num_of_threads, nodes_to_scan_write.len());
        }

        let graph_r = graph.read().unwrap();
        graph_r.clone()
    }
}
