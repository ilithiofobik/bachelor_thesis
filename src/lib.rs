//! # labisu
//! 
//! `labisu` is a library that implements an algorithm for finding large bipartite subgraphs in a graph.
//! It also contains a web crawler that can be used to crawl over the Internet.
//! A structure is provided to represent a graph with undirected edges. 

/// # combinatorics
/// 
/// Module implementing various combinatorial iterators and functions.
pub mod combinatorics;
/// # graphs
/// 
/// Module used to represent a graph with undirected edges.
pub mod graphs;
/// # scraper
/// 
/// Module used to scrape a website for links to other pages.
pub mod scraper;
/// # crawler
/// 
/// Module used to crawl over a net of websites.
pub mod crawler;
/// # bipartite
/// 
/// Module implementing algorithms finding large bipartite subgraphs.
pub mod bipartite;