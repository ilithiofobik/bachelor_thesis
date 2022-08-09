use std::collections::HashMap;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fs::File;

pub struct Graph {
    num_of_nodes: usize,
    num_of_arcs: usize,
    adjacency_list: Vec<HashSet<usize>>, 
    idx_to_name_map: Vec<String>,
    name_to_idx_map: HashMap<String, usize>
}


#[derive(Serialize, Deserialize)]
struct GraphJson {
    num_of_nodes: usize,
    num_of_arcs: usize,
    adjacency_list: Vec<Vec<usize>>,
    names: Vec<String>,
}


impl Graph {
    /// Returns number of nodes.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// assert_eq!(2, e2.get_num_of_nodes());
    /// ```
    pub fn get_num_of_nodes(&self) -> usize {
        self.num_of_nodes
    }

    /// Returns number of arcs.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let mut e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// e2.add_arc_idx(0, 1);
    /// assert_eq!(1, e2.get_num_of_arcs());
    /// ```
    pub fn get_num_of_arcs(&self) -> usize {
        self.num_of_arcs
    }

    /// Returns the name of the node with given index.
    /// The return type is Result which is Ok if the index exists in the graph and Err otherwise.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let e1 = Graph::from_names(vec!["vertex_0".to_string()]);
    /// assert_eq!("vertex_0", e1.idx_to_name(0).unwrap());
    /// ```
    pub fn idx_to_name(&self, idx: usize) -> Result<String, &str> {
        if idx < self.num_of_nodes {
            let name = self.idx_to_name_map[idx].clone();
            Ok(name)
        } else {
            Err("Index is out of bounds.")
        }
    }

    /// Returns the name of the node with given index.
    /// The return type is Result which is Ok if the index exists in the graph and Err otherwise.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let e1 = Graph::from_names(vec!["vertex_0".to_string()]);
    /// assert_eq!(0, e1.name_to_idx("vertex_0").unwrap());
    /// ```
    pub fn name_to_idx(&self, name: &str) -> Result<usize, &str> {
        if self.name_to_idx_map.contains_key(name) {
            let idx = self.name_to_idx_map[name];
            Ok(idx)
        } else {
            Err("There is no node with such name.")
        }
    }

    /// Creates an empty graph. The graph has 0 nodes, empty adjencency list and empty mapping.
    /// # Examples
    /// ```
    /// use bipartite::graphs::Graph;
    /// let empty = Graph::empty();
    /// assert_eq!(0, empty.get_num_of_nodes());
    /// ```
    pub fn empty() -> Graph {
        Graph {
            num_of_nodes: 0,
            num_of_arcs: 0,
            adjacency_list: vec![],
            idx_to_name_map: vec![],
            name_to_idx_map: HashMap::new()
        }
    }

    /// Creates a graph with no arcs based on a vector of nodes names.
    /// # Examples
    /// ```
    /// use bipartite::graphs::Graph;
    /// let e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// assert_eq!(2, e2.get_num_of_nodes());
    /// assert_eq!(0, e2.name_to_idx("vertex_0").unwrap());
    /// assert_eq!("vertex_1", e2.idx_to_name(1).unwrap());
    /// ```
    pub fn from_names(names: Vec<String>) -> Graph {
        let mut name_to_idx_map = HashMap::new(); 
        names.iter().enumerate().for_each(|(idx, name)| { name_to_idx_map.insert(String::from(name), idx); });
        Graph {
            num_of_nodes: names.len(),
            num_of_arcs: 0,
            adjacency_list: vec![HashSet::new(); names.len()],
            idx_to_name_map: names,
            name_to_idx_map
        }
    }

    /// Adds a new node with given name. 
    /// If the name already exists then it is not added.
    /// # Examples
    /// ```
    /// use bipartite::graphs::Graph;
    /// let mut k_one = Graph::empty();
    /// k_one.add_vertex("vertex_0");
    /// k_one.add_vertex("vertex_0");
    /// assert_eq!(1, k_one.get_num_of_nodes());
    /// assert_eq!("vertex_0", k_one.idx_to_name(0).unwrap());
    /// assert_eq!(0, k_one.name_to_idx("vertex_0").unwrap());
    /// ```
    pub fn add_vertex(&mut self, name: &str) {
        if !self.name_to_idx_map.contains_key(name) {
            self.adjacency_list.push(HashSet::new());
            self.name_to_idx_map.insert(String::from(name), self.num_of_nodes);
            self.idx_to_name_map.push(String::from(name));
            self.num_of_nodes += 1;
        }
    }

    /// Returns an iterator on all nodes indices.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let e2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// assert_eq!((0..2), e2.nodes());
    /// ```
    pub fn nodes(&self) -> std::ops::Range<usize> {
        0..self.num_of_nodes
    }

    /// Adds an arc between two nodes based on their indices.
    /// Returns boolean value - if the adding was successful.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_arc_idx(0, 1);
    /// assert!(k2.neighbours_idx(0).unwrap().contains(&1));
    /// ```
    pub fn add_arc_idx(&mut self, from: usize, to: usize) -> bool {
        if from < self.num_of_nodes && to < self.num_of_nodes {
            self.adjacency_list[from].insert(to);
            self.num_of_arcs += 1;
            true
        } else {
            false
        }
    }

    /// Adds an arc between two nodes based on their names.
    /// Returns boolean value - if the adding was successful.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_arc("vertex_0", "vertex_1");
    /// assert!(k2.neighbours_idx(0).unwrap().contains(&1));
    /// ```
    pub fn add_arc(&mut self, from: &str, to: &str) -> bool {
        if self.name_to_idx_map.contains_key(from) && self.name_to_idx_map.contains_key(to) {
            self.adjacency_list[self.name_to_idx_map[from]].insert(self.name_to_idx_map[to]);
            self.num_of_arcs += 1;
            true
        } else {
            false
        }
    }

    /// Lists all neighbours of a given node based on its index.
    /// The return set is the set of indices.
    /// If a given index does not exist in the graph it returns an Err value.
    /// 
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// use std::collections::HashSet;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_arc("vertex_0", "vertex_1");
    /// assert_eq!(k2.neighbours_idx(0), Ok(HashSet::from([1])));
    /// ```
    pub fn neighbours_idx(&self, idx: usize) -> Result<HashSet<usize>, &str> {
        if idx >= self.num_of_nodes {
            Err("Index does not exist in the graph.")
        } else {
            Ok(self.adjacency_list[idx].clone())
        }
    }

    /// Writes a graph to a json file with given filename.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_arc("vertex_0", "vertex_1");
    /// let result = k2.write_to_json(String::from("k2.json"));
    /// assert!(result.is_ok());
    /// ```
    pub fn write_to_json(&self, filename: String) -> serde_json::Result<()> {
        let graph = json!({
            "num_of_nodes": self.num_of_nodes,
            "num_of_arcs": self.num_of_arcs,
            "adjacency_list": self.adjacency_list,
            "names": self.idx_to_name_map.clone()
        }
        );
        serde_json::to_writer(&File::create(filename).unwrap(), &graph)
    }

    /// Reads a graph from a json file with given filename.
    /// # Examples 
    /// ```
    /// use bipartite::graphs::Graph;
    /// let mut k2 = Graph::from_names(vec!["vertex_0".to_string(), "vertex_1".to_string()]);
    /// k2.add_arc("vertex_0", "vertex_1");
    /// let result = k2.write_to_json(String::from("k2.json"));
    /// let read = Graph::read_from_json(String::from("k2.json"));
    /// assert!(result.is_ok());
    /// assert_eq!("dupa", read.to_string());
    /// ```
    pub fn read_from_json(filename: String) -> Graph {
        let data = std::fs::read_to_string(filename)
            .expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&data)
            .expect("JSON does not have correct format.");
        let num_of_nodes = json["num_of_nodes"].as_u64().unwrap() as usize;
        let num_of_arcs = json["num_of_arcs"].as_u64().unwrap() as usize;
        let adjencency_list = json["names"].as_array().clone().unwrap() as Vec<Vec<usize>>;



        Graph {
            num_of_nodes,
            num_of_arcs,
            adjacency_list: vec![HashSet::new(); names.len()],
            idx_to_name_map: names,
            name_to_idx_map
        }
    }

    let mut name_to_idx_map = HashMap::new(); 
    names.iter().enumerate().for_each(|(idx, name)| { name_to_idx_map.insert(String::from(name), idx); });
    Graph {
        num_of_nodes: names.len(),
        num_of_arcs: 0,
        adjacency_list: vec![HashSet::new(); names.len()],
        idx_to_name_map: names,
        name_to_idx_map
    }

   
}