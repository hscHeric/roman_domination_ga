use std::collections::{HashMap, HashSet};

use rand::seq::IteratorRandom;

#[derive(Debug)]
pub struct Graph {
    adjacency_list: Vec<Vec<usize>>, // Lista de adjacências com nós como strings
}

impl Graph {
    pub fn new(num_vertices: usize, edges: &[(usize, usize)]) -> Self {
        let mut adjacency_list = vec![vec![]; num_vertices];
        for &(u, v) in edges {
            adjacency_list[u].push(v);
            adjacency_list[v].push(u);
        }
        Graph { adjacency_list }
    }

    pub fn get_num_vertices(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn new_empty() -> Self {
        Graph {
            adjacency_list: Vec::new(),
        }
    }

    pub fn get_neighbors(&self, vertex: usize) -> &Vec<usize> {
        &self.adjacency_list[vertex]
    }

    pub fn get_vertex_degree(&self, vertex: usize) -> usize {
        self.adjacency_list[vertex].len()
    }

    pub fn h1(&self) -> Vec<u8> {
        let mut f: Vec<u8> = vec![0; self.adjacency_list.len()];
        let mut unvisited: HashSet<usize> = (0..self.adjacency_list.len()).collect();
        let mut rng = rand::thread_rng();

        while !unvisited.is_empty() {
            let &u = unvisited.iter().choose(&mut rng).unwrap();
            f[u] = 2;
            unvisited.remove(&u);

            for &v in self.get_neighbors(u) {
                f[v] = 0;
                unvisited.remove(&v);
            }

            if unvisited.len() == 1 {
                let &last = unvisited.iter().next().unwrap();
                f[last] = 1;
                unvisited.remove(&last);
            }
        }
        f
    }

    pub fn h2(&self) -> Vec<u8> {
        let mut f: Vec<u8> = vec![0; self.adjacency_list.len()];
        let mut unvisited: Vec<usize> = (0..self.adjacency_list.len()).collect();

        // Ordena por grau decrescente
        unvisited.sort_by_key(|&vertex| std::cmp::Reverse(self.get_vertex_degree(vertex)));

        while !unvisited.is_empty() {
            let u = unvisited.remove(0);
            f[u] = 2;

            for &v in self.get_neighbors(u) {
                f[v] = 0;
                unvisited.retain(|&x| x != v);
            }

            if unvisited.len() == 1 {
                let last = unvisited[0];
                f[last] = 1;
                unvisited.clear();
            }
        }
        f
    }
}
