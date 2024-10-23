use core::str;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<char, Vec<char>>, // Lista de adjacências com nós como letras
    labels: HashMap<char, u8>,                // Rótulos numéricos (0, 1 ou 2)
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: char) {
        self.adjacency_list.entry(vertex).or_default();
        self.labels.insert(vertex, 0);
    }

    pub fn add_edge(&mut self, u: char, v: char) {
        self.adjacency_list.entry(u).or_insert(Vec::new()).push(v);
        self.adjacency_list.entry(v).or_insert(Vec::new()).push(u);
    }

    pub fn set_label(&mut self, vertex: char, label: u8) {
        if label > 2 {
            panic!("Invalid label: must be 0, 1 or 2.");
        }
        self.labels.insert(vertex, label);
    }

    pub fn get_label(&self, vertex: char) -> Option<&u8> {
        self.labels.get(&vertex)
    }

    pub fn display(&self) {
        for (vertex, neighbors) in &self.adjacency_list {
            let label = self.labels.get(vertex).unwrap();
            println!(
                "Vertex {}: Neighbors: {:?}, Label: {}",
                vertex, neighbors, label
            );
        }
    }

    pub fn from_file(path: String) -> io::Result<Self> {
        let mut g = Graph::new();
        let path = Path::new(&path);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line_result = line?;
            let line_result = line_result.trim();

            if line_result.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line_result.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(label) = parts[1].parse::<u8>() {
                    let node = parts[0].to_string().chars().next().unwrap();
                    g.add_vertex(node);
                    g.set_label(node, label);
                } else {
                    let u = parts[0].to_string().chars().next().unwrap();
                    let v = parts[1].to_string().chars().next().unwrap();
                    g.add_edge(u, v);
                }
            }
        }

        Ok(g)
    }
}
