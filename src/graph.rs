use std::collections::{HashMap, HashSet};

use rand::seq::IteratorRandom;

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<String, Vec<String>>, // Lista de adjacências com nós como strings
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: String) {
        self.adjacency_list.entry(vertex.clone()).or_default();
    }

    pub fn add_edge(&mut self, u: String, v: String) {
        self.adjacency_list
            .entry(u.clone())
            .or_insert(Vec::new())
            .push(v.clone());
        self.adjacency_list.entry(v).or_insert(Vec::new()).push(u);
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
                let last_vertex = unvisited.iter().next().unwrap().clone();
                f.insert(last_vertex.clone(), 1);
                unvisited.remove(&last_vertex);
            }
        }
        f
    }

    pub fn procedure_h2(&self) -> HashMap<String, u8> {
        let mut f: HashMap<String, u8> = HashMap::new();
        let mut unvisited: Vec<String> = self.adjacency_list.keys().cloned().collect();

        // Ordena o vetor de forma decrescente de acordo com o tamanho da vizinça
        unvisited.sort_by_key(|vertex| {
            std::cmp::Reverse(
                self.adjacency_list
                    .get(vertex)
                    .map_or(0, |neighbors| neighbors.len()),
            )
        }); // Usei clousures, para ordenar o vetor de unvisited pelo grau de cada vértice

        while !unvisited.is_empty() {
            let u = unvisited.remove(0);
            f.insert(u.clone(), 2); // f(u) = 2

            for v in self.neighbors(&u) {
                f.insert(v.clone(), 0);
                unvisited.retain(|x| x != &v);
            }

            if unvisited.len() == 1 {
                let last_vertex = unvisited[0].clone();
                f.insert(last_vertex.clone(), 1);
                unvisited.clear();
            }
        }
        f
    }
}
