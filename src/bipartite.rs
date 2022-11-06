use std::collections::HashSet;

use super::graphs::Graph;
use super::combinatorics::GraySubsets;

struct CountArray {
    highest_degree_vec: Vec<usize>, 
    highest_degree_set: HashSet<usize>, 
    highest_degree_count: usize, 
    subgraph_size: usize, 
    count_array: Vec<usize>,
}

impl CountArray {
    fn new(highest_degree_vec: &Vec<usize>, subgraph_size: usize, g: &Graph) -> CountArray {
        let mut count_array = vec![0; g.get_num_of_vertices()];
        
        (0..subgraph_size).for_each(|i| {
            g.neighbours_idx(highest_degree_vec[i]).unwrap().iter().for_each(|j| {
                count_array[*j] += 1;
            });
        });
        
        CountArray {
            highest_degree_vec: highest_degree_vec.clone(), 
            highest_degree_set: highest_degree_vec.clone().into_iter().collect(), 
            highest_degree_count: highest_degree_vec.len(),
            subgraph_size,
            count_array,
        }
    }

    fn two_bit_change(&mut self, g: &Graph, change: [usize; 2]) {
        g.neighbours_idx(self.highest_degree_vec[change[0]]).unwrap().iter().for_each(|j| {
            self.count_array[*j] -= 1;
        });

        g.neighbours_idx(self.highest_degree_vec[change[1]]).unwrap().iter().for_each(|j| {
            self.count_array[*j] += 1;
        });
    }

    fn is_ok(&self) -> bool {
        let count = 
            self.count_array
            .iter()
            .enumerate()
            .filter(|(idx, c)| **c == self.subgraph_size && !self.highest_degree_set.contains(idx))
            .count();
        
        count >= self.highest_degree_count
    }

    fn solution(&self) -> HashSet<usize> {
        self.count_array
        .iter()
        .enumerate()
        .filter(|(idx, c)| **c == self.subgraph_size && !self.highest_degree_set.contains(idx))
        .map(|(idx, _)| idx)
        .take(self.subgraph_size)
        .collect::<HashSet<usize>>()
    }
}

fn solution_from_curr_subset(curr_subset: &[usize], highest_degree_vertices: &[usize]) -> HashSet<usize> {
    curr_subset
    .iter()
    .enumerate()
    .filter(|(_, is_in)| **is_in == 1) // elements of curr subset
    .map(|(idx, _)| highest_degree_vertices[idx]) // map to the original vertices
    .collect::<HashSet<usize>>()
}

/// Based on algorithm from "Finding bipartite subgraphs efficiently" by Dhruv Mubayi and Gyorgy Turan
/// If the number of edges equals 0, then the algorithm returns two empty sets.
/// The algorithm returns two sets of vertices, which are grouped in two halves of the found complete bipartite graph.
/// # Examples:
/// ```
/// use bipartite::bipartite::find_bipartite;
/// use bipartite::graphs::Graph;
/// use std::collections::HashSet;
/// 
/// let k300 = Graph::complete(300);
/// let (left, right) = find_bipartite(&k300, 10, 3);
/// assert_eq!(left, HashSet::from([0,1,2]));  
/// assert_eq!(right, HashSet::from([10,11,12])); 
/// ```
pub fn find_bipartite(graph: &Graph, highest_degree_size: usize, bipartite_size: usize) -> (HashSet<usize>, HashSet<usize>) {
    let n = graph.get_num_of_vertices();
    let m = graph.get_num_of_edges();
    
    if 0 < m && m.pow(2) < 64 * n.pow(3) {
        for i in graph.vertices() {
            if let Some(j) = graph.neighbours_idx(i).unwrap().iter().next() {
                return (HashSet::from([i]), HashSet::from([*j])); 
            }
        }       
    }

    let highest_degree_vertices = graph.highest_degree_vertices(highest_degree_size);     
    let gray_generator = GraySubsets::new(highest_degree_size, bipartite_size);
    let mut curr_subset = gray_generator.init();
    let mut b = CountArray::new(&highest_degree_vertices, bipartite_size, graph);

    if b.is_ok() {
        let c_set = solution_from_curr_subset(&curr_subset, &highest_degree_vertices);
        let d_set = b.solution();

        return (c_set, d_set);
    }

    for change in gray_generator {
        b.two_bit_change(graph, change);
        curr_subset[change[0]] = 0;
        curr_subset[change[1]] = 1;

        if b.is_ok() {
            let c_set = solution_from_curr_subset(&curr_subset, &highest_degree_vertices);
            let d_set = b.solution();

            return (c_set, d_set)
        }
    }

    (HashSet::new(), HashSet::new()) // no solution
}