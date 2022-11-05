use std::collections::{VecDeque};
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
    must_contain: Vec<String>,
    stop_words: Vec<String>,
}

impl Crawler {
    /// Create a new crawler.
    /// ```
    /// use bipartite::crawler::Crawler;
    /// let parser = Crawler::new("https://pwr.edu.pl/".to_owned(), 1, vec![], vec![]);
    /// ```
    pub fn new(root: String, max_depth: usize, must_contain: Vec<String>, stop_words: Vec<String>) -> Crawler {
        Crawler {
            root,
            max_depth,
            must_contain,
            stop_words,
        }
    }

    /// Crawl the web based on given url and max_depth.
    /// Each url is checked for stop words and must_contain word.
    /// ```
    /// use bipartite::crawler::Crawler;
    /// let crawler = Crawler::new("https://pwr.edu.pl/".to_owned(), 1, vec!["pwr.edu".to_owned()], vec![".txt".to_owned()]);
    /// let links = crawler.crawl();
    /// links.write_to_json("testing3.json").unwrap(); // to be deleted
    /// assert_eq!(links.idx_to_name(0).unwrap(), "https://pwr.edu.pl/");
    /// for idx in links.vertices().skip(1) {
    ///     assert!(links.idx_to_name(idx).unwrap().contains("pwr.edu"));
    ///     assert!(!links.idx_to_name(idx).unwrap().contains(".txt"));
    ///     assert_ne!(links.idx_to_name(idx).unwrap(), "https://pwr.edu.pl/");
    /// }
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
            
            (0..num_of_threads)
            .into_iter()
            .for_each(|_| scrapers.push(Arc::new(Scraper::new(self.must_contain.clone(),
                                                                      self.stop_words.clone()))));
            
            scrapers
        }; // scrapers are used but not changed     
        
        let mut curr_num_of_threads = 1;
        while curr_num_of_threads > 0 {
            let mut threads = Vec::with_capacity(curr_num_of_threads);
            let (tx, rx) = mpsc::channel();

            for (queue_idx, scraper_arc) in scrapers.iter().enumerate().take(curr_num_of_threads) {
                let scraper = Arc::clone(scraper_arc);
                let graph = Arc::clone(&graph);
                let nodes_to_scan_clone = Arc::clone(&nodes_to_scan);
                let max_depth = Arc::clone(&max_depth);
                let tx = tx.clone();

                threads.push(std::thread::spawn(move || {
                    let graph = graph.read().unwrap();
                    let (depth, node_id) = *nodes_to_scan_clone.read().unwrap().get(queue_idx).unwrap();
                    let root_node_name = graph.idx_to_name(node_id).unwrap();
                    let links = scraper.scrape(&root_node_name);
                    let links = 
                        if depth == *max_depth {
                            links.into_iter().filter(|link| graph.contains_vertex(link)).collect()
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

                    tx.send((queue_idx, links)).unwrap();
                }));
            }

            for thread in threads {
                thread.join().expect("The thread creating or execution failed!");
            }

            let mut graph_write = graph.write().unwrap();
            let mut nodes_to_scan_write = nodes_to_scan.write().unwrap();

            for _ in 0..curr_num_of_threads {
                let (queue_idx, links) = rx.recv().unwrap();
                let (depth, node_id) = *nodes_to_scan_write.get(queue_idx).unwrap();
                let root_node_name = graph_write.idx_to_name(node_id).unwrap();
                for link in links {
                    match link {
                        Index::StrIndex(link) => {
                            let graph_idx = graph_write.name_to_idx(&link);
                            match graph_idx {
                                Ok(link_id) => {
                                    graph_write.add_edge_idx(node_id, link_id);
                                },
                                Err(_) => {
                                    graph_write.add_vertex(&link);
                                    let link_id = graph_write.get_num_of_vertices() - 1;
                                    graph_write.add_edge(&root_node_name, &link);
                                    nodes_to_scan_write.push_back((depth + 1, link_id));
                                }
                            }
                        },
                        Index::NumIndex(link_id) => {
                            graph_write.add_edge_idx(node_id, link_id);
                        }
                    }
                }
            }

            // deleting scanned nodes
            (0..curr_num_of_threads).into_iter().for_each(|_| {
                nodes_to_scan_write.pop_front();
            });

            curr_num_of_threads = std::cmp::min(num_of_threads, nodes_to_scan_write.len());
        }

        let graph_r = graph.read().unwrap();
        graph_r.clone()
    }
}
