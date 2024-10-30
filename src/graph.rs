use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

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
                if unvisited.contains(&v) {
                    f[v] = 0;
                    unvisited.remove(&v);
                }
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

        unvisited.sort_by_key(|&vertex| std::cmp::Reverse(self.get_vertex_degree(vertex)));

        while !unvisited.is_empty() {
            let u = unvisited.remove(0);
            f[u] = 2;

            for &v in self.get_neighbors(u) {
                if let Some(pos) = unvisited.iter().position(|&x| x == v) {
                    f[v] = 0;
                    unvisited.remove(pos);
                }
            }

            if unvisited.len() == 1 {
                let last = unvisited[0];
                f[last] = 1;
                unvisited.clear();
            }
        }
        f
    }

    pub fn from_file(file_path: String) -> io::Result<Self> {
        let file = File::open(&file_path)?;
        let reader = io::BufReader::new(file);
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut num_vertices = 0;

        for line in reader.lines() {
            let line = line?;
            let vertices: Vec<usize> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            if vertices.len() == 2 {
                let (u, v) = (vertices[0], vertices[1]);
                edges.push((u, v));
                num_vertices = num_vertices.max(u + 1).max(v + 1);
            }
        }

        Ok(Graph::new(num_vertices, &edges))
    }

    pub fn get_graph_size(&self) -> usize {
        let mut edge_count = 0;
        for v in &self.adjacency_list {
            edge_count += v.len();
        }
        edge_count / 2
    }
}
