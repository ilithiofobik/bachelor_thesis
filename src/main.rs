    use bipartite::crawler::Crawler;

    fn main() {
        let crawler = Crawler::new(
            "https://pwr.edu.pl/".to_owned(), 
            20, 
            vec!["pwr.edu".to_owned()], 
            vec![".txt".to_owned(), ".pdf".to_owned()]
        );
        let links = crawler.crawl();
        links.write_to_json("testing20.json").unwrap(); // to be deleted
    }