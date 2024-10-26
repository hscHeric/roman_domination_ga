use std::env;

use graph::Graph;

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

    // Define rótulos
    graph.set_label("A".to_string(), 0);
    graph.set_label("B".to_string(), 2);
    graph.set_label("C".to_string(), 0);
    graph.set_label("D".to_string(), 1);

    // Exibe o grafo
    println!("Grafo:");
    graph.display();

    // Executa a procedure_h1
    let result_h1 = graph.procedure_h1();
    println!("\nResultado da Procedure H1:");
    for (vertex, label) in result_h1 {
        println!("Vertex: {}, Label: {}", vertex, label);
    }

    // Executa a procedure_h2
    let result_h2 = graph.procedure_h2();
    println!("\nResultado da Procedure H2:");
    for (vertex, label) in result_h2 {
        println!("Vertex: {}, Label: {}", vertex, label);
    }

    // Verifica se o grafo é dominador romano
    let is_roman = graph.is_roman_dominating_graph();
    println!("\nO grafo é um grafo de dominação romana? {}", is_roman);
}
