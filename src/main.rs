use std::time::Instant;
use bipartite::graphs::Graph;
use bipartite::bipartite::{find_bipartite, qr_parameters};

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    for filename in std::fs::read_dir(current_dir).unwrap() {
        if let Ok(name) = filename {
            let name = name.file_name().into_string().unwrap();
            if name.ends_with(".json") && name.starts_with("random") {
                let result_filename = format!("result_{}", name);
                let k_n = Graph::read_from_json(&name);
                let (q, r) = qr_parameters(&k_n);
                let start = Instant::now();
                let (set1, set2) = find_bipartite(&k_n, 2, 2);
                let duration = start.elapsed();
                let result = format!("duration={:?}, q={}, r={}, set1={}, set2={}", duration, q, r, set1.len(), set2.len());
                std::fs::write(result_filename, result).expect("Unable to write file");
                println!("Done {} in {:?}", name, duration);
            }
        }
    }
}