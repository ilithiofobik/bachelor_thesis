use std::time::Instant;
use bipartite::graphs::GraphVec;

fn main() {
    for n in (1..=10).map(|x| 10000 * x) {
        let m = (8.0 * (n as f64).powf(1.5)).ceil() as usize + 1;

        let start = Instant::now();
        let k_n = GraphVec::complete(n);
        let filename = format!("k_n{}.json", n);
        k_n.write_to_json(&filename).unwrap();
        let duration = start.elapsed();
        println!("Done {} in {:?}", filename, duration);
    
        for v in 0..10 {
            let start = Instant::now();
            let g = GraphVec::random_given_edges(n, m);
            let filename = format!("random_n{}_m{}_version{}.json", n, m, v);
            g.write_to_json(&filename).unwrap();
            let duration = start.elapsed();
            println!("Done {} in {:?}", filename, duration);
        }
    }

    // Complete graphs
    for n in (2..=10).map(|x| 1000 * x) {
        let start = Instant::now();
        let k_n = GraphVec::complete(n);
        let filename = format!("k_n{}.json", n);
        k_n.write_to_json(&filename).unwrap();
        let duration = start.elapsed();
        println!("Done {} in {:?}", filename, duration);
    }
}