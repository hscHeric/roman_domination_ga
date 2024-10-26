use core::str;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use rand::seq::IteratorRandom;

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<String, Vec<String>>, // Lista de adjacências com nós como strings
    labels: HashMap<String, u8>,                  // Rótulos numéricos (0, 1 ou 2)
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: String) {
        self.adjacency_list.entry(vertex.clone()).or_default();
        self.labels.insert(vertex, 0);
    }

    pub fn add_edge(&mut self, u: String, v: String) {
        self.adjacency_list
            .entry(u.clone())
            .or_insert(Vec::new())
            .push(v.clone());
        self.adjacency_list.entry(v).or_insert(Vec::new()).push(u);
    }

    pub fn set_label(&mut self, vertex: String, label: u8) {
        if label > 2 {
            panic!("Invalid label: must be 0, 1, or 2.");
        }
        self.labels.insert(vertex, label);
    }

    pub fn get_label(&self, vertex: &String) -> Option<&u8> {
        self.labels.get(vertex)
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
                    let node = parts[0].to_string();
                    g.add_vertex(node.clone());
                    g.set_label(node, label);
                } else {
                    let u = parts[0].to_string();
                    let v = parts[1].to_string();
                    g.add_edge(u, v);
                }
            }
        }

        Ok(g)
    }

    pub fn is_roman_dominating_graph(&self) -> bool {
        for (k, v) in &self.adjacency_list {
            if *self.get_label(k).unwrap() == 0 {
                let mut found_adjacent_two = false;
                for adj in v {
                    if let Some(&label) = self.get_label(adj) {
                        if label == 2 {
                            found_adjacent_two = true;
                            break; // Não precisamos verificar outros vizinhos, já achamos um 2
                        }
                    }
                }
                if !found_adjacent_two {
                    return false; // Se não encontrou nenhum adjacente com rótulo 2
                }
            }
        }

        true // Se passou por todos os vértices sem problemas
    }

    pub fn neighbors(&self, vertex: &String) -> Vec<String> {
        self.adjacency_list.get(vertex).cloned().unwrap_or_default()
    }

    pub fn procedure_h1(&self) -> HashMap<String, u8> {
        // `f` representa nossa função de dominação romana
        let mut f: HashMap<String, u8> = HashMap::new();
        let mut unvisited: HashSet<String> = self.adjacency_list.keys().cloned().collect();
        let mut rng = rand::thread_rng();

        while !unvisited.is_empty() {
            // Seleciona um vértice `u` de `unvisited` aleatoriamente
            let u = unvisited.iter().choose(&mut rng).unwrap().clone();

            // Atribui f(u) = 2 e remove `u` do conjunto `unvisited`
            f.insert(u.clone(), 2);
            unvisited.remove(&u);

            // Para cada vizinho `v` de `u`, define f(v) = 0 e remove `v` de `unvisited`
            for v in self.neighbors(&u) {
                f.insert(v.clone(), 0);
                unvisited.remove(&v);
            }

            if unvisited.len() == 1 {
                for v in unvisited.iter() {
                    f.insert(v.clone(), 1);
                }
            }
        }
        f
    }
}
