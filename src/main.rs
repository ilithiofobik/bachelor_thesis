use std::time::Instant;
use bipartite::graphs::Graph;
use bipartite::bipartite::{find_bipartite, qr_parameters};

fn main() {
    // Complete graphs
    for n in (1..=10).map(|x| 1000 * x) {
        let filename = format!("k_n{}.json", n);
        let result_filename = format!("result_{}", filename);
        let k_n = Graph::read_from_json(&filename);
        let (q, r) = qr_parameters(&k_n);
        let start = Instant::now();
        let (set1, set2) = find_bipartite(&k_n, 2, 2);
        let duration = start.elapsed();
        let result = format!("duration={:?}, q={}, r={}, set1={}, set2={}", duration, q, r, set1.len(), set2.len());
        std::fs::write(result_filename, result).expect("Unable to write file");
        println!("Done {} in {:?}", filename, duration);
    }
}