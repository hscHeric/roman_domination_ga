use std::env;

use graph::Graph;

mod genetic_algorithm;
mod graph;

fn main() {
    // Cria um novo grafo
    let mut graph = Graph::new();

    // Adiciona vértices
    graph.add_vertex("A".to_string());
    graph.add_vertex("B".to_string());
    graph.add_vertex("C".to_string());
    graph.add_vertex("D".to_string());

    // Adiciona arestas
    graph.add_edge("A".to_string(), "B".to_string());
    graph.add_edge("A".to_string(), "C".to_string());
    graph.add_edge("B".to_string(), "D".to_string());
}
