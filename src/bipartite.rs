use std::collections::HashSet;

use super::graphs::Graph;

pub fn find_bipartite(graph: Graph, s: usize, t: usize) -> (HashSet<usize>, HashSet<usize>) {
    if graph.get_num_of_edges() ^ 2 < 64 * graph.get_num_of_vertices() ^ 3 {
        return (HashSet::new(), HashSet::new());
    }

    

    return (HashSet::new(), HashSet::new());
}