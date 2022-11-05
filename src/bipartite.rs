use std::collections::HashSet;

use super::graphs::Graph;
use super::combinatorics::GraySubsets;

pub struct CountArray {
    highest_degree_vec: Vec<usize>, 
    highest_degree_set: HashSet<usize>, 
    highest_degree_count: usize, 
    subgraph_size: usize, 
    count_array: Vec<usize>,
}

impl CountArray {
    /// Creates a new CountArray object.
    /// The object contains current subgraph and counts the neighbours of the current subgraph.
    /// # Examples:
    /// ```
    /// use bipartite::bipartite::CountArray;
    /// use bipartite::graphs::Graph;
    /// use std::collections::HashSet;
    /// 
    /// let k6 = Graph::complete(6);
    /// let highest_degree_vec = k6.highest_degree_vertices(3);
    /// let mut count_array = CountArray::new(&highest_degree_vec, 2, &k6);
    /// assert!(count_array.is_ok()); // found graph K_2,2
    /// assert_eq!(count_array.solution(), HashSet::from([3,4])); // found graph K_2,2
    /// ```
    pub fn new(highest_degree_vec: &Vec<usize>, subgraph_size: usize, g: &Graph) -> CountArray {
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

    pub fn two_bit_change(&mut self, g: &Graph, change: [usize; 2]) -> () {
        g.neighbours_idx(self.highest_degree_vec[change[0]]).unwrap().iter().for_each(|j| {
            self.count_array[*j] -= 1;
        });

        g.neighbours_idx(self.highest_degree_vec[change[1]]).unwrap().iter().for_each(|j| {
            self.count_array[*j] += 1;
        });
    }

    pub fn is_ok(&self) -> bool {
        let count = 
            self.count_array
            .iter()
            .enumerate()
            .filter(|(idx, c)| **c == self.subgraph_size && !self.highest_degree_set.contains(idx))
            .count();
        
        count >= self.highest_degree_count
    }

    pub fn solution(&self) -> HashSet<usize> {
        self.count_array
        .iter()
        .enumerate()
        .filter(|(idx, c)| **c == self.subgraph_size && !self.highest_degree_set.contains(idx))
        .map(|(idx, _)| idx)
        .take(self.subgraph_size)
        .collect::<HashSet<usize>>()
    }
}

/// Based on algorithm from "Finding bipartite subgraphs efficiently" by Dhruv Mubayi and Gyorgy Turan
/// If the number of edges equals 0, then the algorithm does not work properly.
/// 
/// 
pub fn find_bipartite(graph: Graph, s: usize, t: usize) -> (HashSet<usize>, HashSet<usize>) {
    let n = graph.get_num_of_vertices();
    let m = graph.get_num_of_edges();
    
    if 0 < m && m ^ 2 < 64 * (n ^ 3) {
        for i in 0..n {
            if !graph.neighbours_idx(i).unwrap().is_empty() {
                let j = graph.neighbours_idx(i).unwrap().iter().next().unwrap().clone();
                return (HashSet::from([i]), HashSet::from([j])); 
            }
        }       
    }

    let r = graph.highest_degree_vertices(s); 
    
    let gray_generator = GraySubsets::new(s, t);
    let mut curr_subset = gray_generator.init();
    let mut b = CountArray::new(&curr_subset, t, &graph);

    if b.is_ok() {
        let c_set = 
            curr_subset
            .into_iter()
            .enumerate()
            .filter(|(_, is_in)| *is_in == 1) // elements of curr subset
            .map(|(idx, _)| r[idx]) // map to the original vertices
            .collect::<HashSet<usize>>();

        let d_set = b.solution();

        return (c_set, d_set);
    }

    for change in gray_generator {
        b.two_bit_change(&graph, change);
        curr_subset[change[0]] = 0;
        curr_subset[change[1]] = 1;

        if b.is_ok() {
            let c_set = 
                curr_subset
                .into_iter()
                .enumerate()
                .filter(|(_, is_in)| *is_in == 1) // elements of curr subset
                .map(|(idx, _)| r[idx]) // map to the original vertices
                .collect::<HashSet<usize>>();

            let d_set = b.solution();

            return (c_set, d_set)
        }
    }

    (HashSet::new(), HashSet::new()) // no solution
}