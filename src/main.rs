use genetic_algorithm::RomanDominationGA;
use graph::Graph;

mod genetic_algorithm;
mod graph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./data/edges/pores_1_edges.txt".to_string();

    // Tenta criar um grafo a partir do arquivo
    let graph = Graph::from_file(file_path)?;
    let mut rdga = RomanDominationGA::new(graph, Some(10));
    let a = rdga.run(100, 10, 10, 0.95);
    println!("{}", a.fitness.unwrap());
    Ok(())
}
