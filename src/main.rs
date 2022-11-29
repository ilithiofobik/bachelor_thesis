use bipartite::crawler::Crawler;

fn main() {
    let crawler = Crawler::new(
        "https://pwr.edu.pl/".to_owned(), 
        10, 
        vec!["pwr.edu".to_owned()], 
        vec![".txt".to_owned(), ".pdf".to_owned(), ".json".to_owned(), ".xml".to_owned()]
    );
    let links = crawler.crawl();
    links.write_to_json("pwr10.json").unwrap(); // to be deleted
}