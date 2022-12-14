use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

/// Enum used to color graph's vertices.
#[derive(Clone, PartialEq)]
enum Color {
    Black,
    White,
    Gray
}

impl Color {
    /// Returns the opposite color of the current color.
    fn reverse(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::Gray => Color::Gray
        }
    }
}

/// A structure representing an undirected graph, where vertices are named by strings.
#[derive(Clone)]
pub struct Graph {
    num_of_vertices: usize,
    num_of_edges: usize,
    neighbours: Vec<HashSet<usize>>,
    idx_to_name_map: Vec<String>,
    name_to_idx_map: HashMap<String, usize>,
}

/// A structure representing a graph that can be easily
/// tranformed into a json file.
#[derive(Serialize, Deserialize)]
struct GraphJson {
    num_of_vertices: usize,
    num_of_edges: usize,
    neighbours: Vec<Vec<usize>>,
    names: Vec<String>,
}

impl Graph {    
    /// Returns number of vertices.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// assert_eq!(2, e2.get_num_of_vertices());
    /// ```
    pub fn get_num_of_vertices(&self) -> usize {
        self.num_of_vertices
    }

    /// Returns number of edges.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// e2.add_edge_idx(0, 1);
    /// assert_eq!(1, e2.get_num_of_edges());
    /// ```
    pub fn get_num_of_edges(&self) -> usize {
        self.num_of_edges
    }

    /// Returns the name of the vertex with given index.
    /// The return type is Option which is Some if the index exists in the graph and None otherwise.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let e1 = Graph::from_names(vec!["vertex_0".to_string()]);
    /// assert_eq!(Some("vertex_0".to_string()), e1.idx_to_name(0));
    /// assert_eq!(None, e1.idx_to_name(1));
    /// ```
    pub fn idx_to_name(&self, idx: usize) -> Option<String> {
        self.idx_to_name_map.get(idx).cloned()
    }

    /// Returns the index of the vertex with given name.
    /// The return type is Option which is Some if the index exists in the graph and None otherwise.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let e1 = Graph::from_names(vec!["vertex_0".to_string()]);
    /// assert_eq!(Some (0), e1.name_to_idx("vertex_0"));
    /// assert_eq!(None, e1.name_to_idx("vertex_1"));
    /// ```
    pub fn name_to_idx(&self, name: &str) -> Option<usize> {
        self.name_to_idx_map.get(name).cloned()
    }

    /// Creates an empty graph. The graph has 0 vertices, empty adjacency list and empty mapping.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let empty = Graph::empty();
    /// assert_eq!(0, empty.get_num_of_vertices());
    /// ```
    pub fn empty() -> Graph {
        Graph {
            num_of_vertices: 0,
            num_of_edges: 0,
            neighbours: vec![],
            idx_to_name_map: vec![],
            name_to_idx_map: HashMap::new(),
        }
    }

    /// Creates a random graph with given number of vertices.
    /// Each edge has a probability of ppb to be present.
    /// i-th vertex is named "vertex_i".
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let k2 = Graph::random(2, 1.0);
    /// assert!(k2.neighbours_idx(1).unwrap().contains(&0));
    /// assert!(k2.neighbours_idx(0).unwrap().contains(&1));
    /// ```
    pub fn random(num_of_vertices: usize, ppb: f64) -> Graph {
        let mut graph = Graph::empty();
        for i in 0..num_of_vertices {
            graph.add_vertex(&format!("vertex_{}", i));
        }
        let mut rand_thread = rand::thread_rng();
        for from in 0..num_of_vertices {
            for to in from + 1..num_of_vertices {
                if rand_thread.gen_range(0.0..1.0) < ppb {
                    graph.add_edge_idx(from, to);
                }
            }
        }
        graph
    }

    /// Creates a random graph with given number of vertices and edges.
    /// Each edge has the same probability of being present.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let rand42 = Graph::random_given_edges(42, 42);
    /// assert_eq!(rand42.get_num_of_edges(), 42);
    /// assert_eq!(rand42.get_num_of_vertices(), 42);
    /// ```
    pub fn random_given_edges(num_of_vertices: usize, num_of_edges: usize) -> Graph {
        let mut graph = Graph::empty();
        for i in 0..num_of_vertices {
            graph.add_vertex(&format!("v{}", i));
        }

        let mut edges_list =
            (0..num_of_vertices)
            .flat_map(|from| (from + 1..num_of_vertices).map(move |to| (from, to)))
            .collect::<Vec<(usize, usize)>>();

        let potential_edges_count = edges_list.len();

        let mut rand_thread = rand::thread_rng();
        
        for from in 0..num_of_edges - 2 {
            let to = rand_thread.gen_range(from + 1..potential_edges_count);
            edges_list.swap(from, to);
        }

        (0..num_of_edges)
        .map(|i| edges_list[i])
        .for_each(|(from, to)| { graph.add_edge_idx(from, to); });

        graph
    }

    /// Creates a complete graph with given number of vertices.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let k2 = Graph::complete(2);
    /// assert!(k2.neighbours_idx(1).unwrap().contains(&0));
    /// assert!(k2.neighbours_idx(0).unwrap().contains(&1));
    /// ```
    pub fn complete(num_of_vertices: usize) -> Graph {
        let num_of_edges = num_of_vertices * (num_of_vertices - 1) / 2;
        let neighbours =
            (0..num_of_vertices)
            .into_iter()
            .map(|i|  (0..num_of_vertices).filter(|j| i != *j).collect::<HashSet<usize>>())
            .collect();

        let idx_to_name_map = (0..num_of_vertices).into_iter().map(|i| format!("vertex_{}", i)).collect();
        let name_to_idx_map = (0..num_of_vertices).into_iter().map(|i| (format!("vertex_{}", i), i)).collect();

        Graph {
            num_of_vertices,
            num_of_edges,
            neighbours,
            idx_to_name_map,
            name_to_idx_map,
        }
    }


    /// Creates a graph with no edges based on a vector of vertices names.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// assert_eq!(2, e2.get_num_of_vertices());
    /// assert_eq!(0, e2.name_to_idx("vertex_0").unwrap());
    /// assert_eq!("vertex_1", e2.idx_to_name(1).unwrap());
    /// ```
    pub fn from_names(names: Vec<String>) -> Graph {
        let mut name_to_idx_map = HashMap::new();
        names.iter().enumerate().for_each(|(idx, name)| {
            name_to_idx_map.insert(String::from(name), idx);
        });
        Graph {
            num_of_vertices: names.len(),
            num_of_edges: 0,
            neighbours: vec![HashSet::new(); names.len()],
            idx_to_name_map: names,
            name_to_idx_map,
        }
    }

    /// Adds a new vertex with given name.
    /// If the name already exists then it is not added.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut k_one = Graph::empty();
    /// k_one.add_vertex("vertex_0");
    /// k_one.add_vertex("vertex_0");
    /// assert_eq!(1, k_one.get_num_of_vertices());
    /// assert_eq!("vertex_0", k_one.idx_to_name(0).unwrap());
    /// assert_eq!(0, k_one.name_to_idx("vertex_0").unwrap());
    /// ```
    pub fn add_vertex(&mut self, name: &str) {
        if !self.name_to_idx_map.contains_key(name) {
            self.neighbours.push(HashSet::new());
            self.name_to_idx_map
                .insert(String::from(name), self.num_of_vertices);
            self.idx_to_name_map.push(String::from(name));
            self.num_of_vertices += 1;
        }
    }

    /// Removes a vertex with given name.
    /// If the name does not exist then nothing happens.
    /// Returns true if the vertex was removed.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// use std::collections::HashSet;
    /// let mut k5 = Graph::complete(5);
    /// assert!(k5.remove_vertex("vertex_2"));
    /// assert!(!k5.remove_vertex("vertex_2"));
    /// assert_eq!(4, k5.get_num_of_vertices());
    /// assert_eq!(6, k5.get_num_of_edges());
    /// assert_eq!(k5.neighbours_idx(0).unwrap(), HashSet::from([1, 2, 3]));
    /// ```
    pub fn remove_vertex(&mut self, name: &str) -> bool {
        if let Some(idx) = self.name_to_idx(name) {
            self.num_of_vertices -= 1;
            self.num_of_edges -= self.neighbours[idx].len();

            self.neighbours.remove(idx);
            self.neighbours =
                self.neighbours
                .clone()
                .into_iter()
                .map(|mut neighbours| { 
                    neighbours.remove(&idx); 
                    neighbours.into_iter().map(|neighbour| if neighbour > idx { neighbour - 1 } else { neighbour }).collect() 
                })
                .collect();

            self.idx_to_name_map.remove(idx);
            self.name_to_idx_map.remove(name);

            self.name_to_idx_map =
                self.name_to_idx_map
                .clone()
                .into_iter()
                .map(|(s, i)| if i > idx { (s, i - 1) } else { (s, i) })
                .collect();
            
            return true 
        } 
        false
    }

    /// Returns an iterator on all vertices indices.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// assert_eq!((0..2), e2.vertices());
    /// ```
    pub fn vertices(&self) -> std::ops::Range<usize> {
        0..self.num_of_vertices
    }

    /// Returns an iterator on all vertices names.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let e1 = Graph::from_names(vec!["vertex_0".to_string()]);
    /// assert!(e1.contains_vertex("vertex_0"));
    /// assert!(!e1.contains_vertex("vertex_1"));
    /// ```
    pub fn contains_vertex(&self, name: &str) -> bool {
        self.name_to_idx_map.contains_key(name)
    }

    /// Adds an edge between two vertices based on their indices.
    /// Returns boolean value - if the adding was successful.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_edge_idx(0, 1);
    /// assert!(k2.neighbours_idx(0).unwrap().contains(&1));
    /// assert!(k2.neighbours_idx(1).unwrap().contains(&0));
    /// ```
    pub fn add_edge_idx(&mut self, from: usize, to: usize) -> bool {
        if from < self.num_of_vertices 
        && to < self.num_of_vertices 
        && from != to
        && !self.neighbours[from].contains(&to) {
            self.neighbours[to].insert(from);
            self.neighbours[from].insert(to);
            self.num_of_edges += 1;
            true
        } else {
            false
        }
    }

    /// Adds an edge between two vertices based on their names.
    /// Returns boolean value - if the adding was successful.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_edge("vertex_0", "vertex_1");
    /// assert!(k2.neighbours_idx(0).unwrap().contains(&1));
    /// assert!(k2.neighbours_idx(1).unwrap().contains(&0));
    /// ```
    pub fn add_edge(&mut self, from: &str, to: &str) -> bool {
        if let (Some (from_idx), Some (to_idx)) = (self.name_to_idx(from), self.name_to_idx(to)) {
            if from != to && !self.neighbours[from_idx].contains(&to_idx) {
                self.neighbours[to_idx].insert(from_idx);
                self.neighbours[from_idx].insert(to_idx);
                self.num_of_edges += 1;
                return true
            }
        } 
        false
    }

    /// Lists all neighbours of a given vertex based on its index.
    /// The return set is the set of indices.
    /// If a given index does not exist in the graph it returns an Err value.
    ///
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// use std::collections::HashSet;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_edge("vertex_0", "vertex_1");
    /// assert_eq!(k2.neighbours_idx(0), Ok(HashSet::from([1])));
    /// assert_eq!(k2.neighbours_idx(1), Ok(HashSet::from([0])));
    /// assert_eq!(k2.neighbours_idx(2), Err("Index does not exist in the graph."));
    /// ```
    pub fn neighbours_idx(&self, idx: usize) -> Result<HashSet<usize>, &str> {
        if idx >= self.num_of_vertices {
            Err("Index does not exist in the graph.")
        } else {
            Ok(self.neighbours[idx].clone())
        }
    }


    /// Returns sorted list of s vertices with highest degree.
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut g4 = Graph::from_names(vec!["v_0".to_string(), "v_1".to_string(), "v_2".to_string(), "v_3".to_string()]); 
    /// g4.add_edge("v_3", "v_0");
    /// g4.add_edge("v_3", "v_1");
    /// g4.add_edge("v_3", "v_2");
    /// g4.add_edge("v_2", "v_1");
    /// assert_eq!(g4.highest_degree_vertices(3), vec![3, 1, 2]);
    /// 
    pub fn highest_degree_vertices(&self, s: usize) -> Vec<usize> {
        let mut vertices = self.vertices().collect::<Vec<usize>>();
        vertices.sort_by(|a, b| self.neighbours[*b].len().cmp(&self.neighbours[*a].len()));
        vertices[0..s].to_vec()
    }

    /// Returns sorted list of s vertices with lowest degree.
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut g4 = Graph::from_names(vec!["v_0".to_string(), "v_1".to_string(), "v_2".to_string(), "v_3".to_string()]); 
    /// g4.add_edge("v_3", "v_0");
    /// g4.add_edge("v_3", "v_1");
    /// g4.add_edge("v_3", "v_2");
    /// g4.add_edge("v_2", "v_1");
    /// assert_eq!(g4.lowest_degree_vertices(3), vec![0, 1, 2]);
    /// 
    pub fn lowest_degree_vertices(&self, s: usize) -> Vec<usize> {
        let mut vertices = self.vertices().collect::<Vec<usize>>();
        vertices.sort_by(|a, b| self.neighbours[*a].len().cmp(&self.neighbours[*b].len()));
        vertices[0..s].to_vec()
    }

    /// Writes a graph to a json file with given filename.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_edge("vertex_0", "vertex_1");
    /// let result = k2.write_to_json("k2.json");
    /// assert!(result.is_ok());
    /// ```
    pub fn write_to_json(&self, filename: &str) -> serde_json::Result<()> {
        let graph = json!({
            "num_of_vertices": self.num_of_vertices,
            "num_of_edges": self.num_of_edges,
            "neighbours": self.neighbours,
            "names": self.idx_to_name_map
        }
        );
        serde_json::to_writer(&File::create(filename).unwrap(), &graph)
    }

    /// Reads a graph from a json file with given filename.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_edge("vertex_0", "vertex_1");
    /// k2.write_to_json("k2.json").unwrap();
    /// let read = Graph::read_from_json("k2.json");
    /// assert_eq!(2, read.get_num_of_vertices());
    /// assert_eq!(1, read.get_num_of_edges());
    /// assert_eq!("vertex_0", read.idx_to_name(0).unwrap());
    /// assert_eq!(1, read.name_to_idx("vertex_1").unwrap());
    /// ```
    pub fn read_from_json(filename: &str) -> Graph {
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        let json: serde_json::Value =
            serde_json::from_str(&data).expect("JSON does not have correct format.");

        let num_of_vertices = json["num_of_vertices"].as_u64().unwrap() as usize;

        let num_of_edges = json["num_of_edges"].as_u64().unwrap() as usize;

        let neighbours = json["neighbours"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| {
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_u64().unwrap() as usize)
                    .collect::<HashSet<usize>>()
            })
            .collect::<Vec<HashSet<usize>>>();

        let names: Vec<String> = json["names"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| String::from(value.as_str().unwrap()))
            .collect();

        let mut name_to_idx_map = HashMap::new();
        names.iter().enumerate().for_each(|(idx, name)| {
            name_to_idx_map.insert(String::from(name), idx);
        });

        Graph {
            num_of_vertices,
            num_of_edges,
            neighbours,
            idx_to_name_map: names,
            name_to_idx_map,
        }
    }

    /// Checks if a graph is bipartite.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut k2 = Graph::complete(2);
    /// let mut k3 = Graph::complete(3);
    /// assert!(k2.is_bipartite());
    /// assert!(!k3.is_bipartite());
    /// ```
    pub fn is_bipartite(&self) -> bool {
        let mut color = vec![Color::Gray; self.num_of_vertices];
        let mut stack = Vec::new();
        for idx in 0..self.num_of_vertices {
            if color[idx] == Color::Gray {
                color[idx] = Color::Black;
                stack.push(idx);
                while !stack.is_empty() {
                    let current = stack.pop().unwrap(); 
                    for neighbour in &self.neighbours[current] {
                        if color[*neighbour] == Color::Gray {
                            color[*neighbour] = color[current].reverse();
                            stack.push(*neighbour);
                        } else if color[*neighbour] == color[current] {
                            return false
                        }
                    }
                }
            }
        }
        true
    }

    /// Removes vertices until the graph satisfies |E| ^ 2 > 64 * |V| ^ 3
    /// Returns boolean value saying if the reduction was successful.
    /// # Examples
    /// ```
    /// use labisu::graphs::Graph;
    /// let mut k256 = Graph::complete(256);
    /// let mut rand1024_with_leaf = Graph::random_given_edges(1024, 262145);
    /// rand1024_with_leaf.add_vertex("leaf");
    /// rand1024_with_leaf.add_edge_idx(1024, 0);
    /// assert!(rand1024_with_leaf.reduce_to_dense());
    /// assert!(!k256.reduce_to_dense());
    pub fn reduce_to_dense(&mut self) -> bool {
        let lowest_degree_vertices: Vec<String> = 
            self.lowest_degree_vertices(self.num_of_vertices)
            .into_iter()
            .map(|idx| self.idx_to_name(idx).unwrap())
            .collect();

        for name in lowest_degree_vertices {
            let n = self.get_num_of_vertices();
            let m = self.get_num_of_edges();

            println!("n: {}, m: {}", n, m);

            if m.pow(2) > 64 * n.pow(3) {
                return true
            }
            self.remove_vertex(&name);
        }
        false
    }
}