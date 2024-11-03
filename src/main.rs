use genetic_algorithm::RomanDominationGA;
use graph::Graph;

mod genetic_algorithm;
mod graph;

use std::env;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Uso: {} <file_path> <trials>", args[0]);
        exit(1);
    }

    let file_path = args[1].clone();

    const MAX_STAGNANT: usize = 100;
    const GENERATIONS: usize = 1000;
    const TOURNMENT_SIZE: usize = 2;
    const CROSSOVER_PROBABILITY: f32 = 0.9;
    let file_name = Path::new(&file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("arquivo");

    let trials: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Erro: 'trials' deve ser um número inteiro positivo.");
            exit(1);
        }
    };

    let graph = Graph::from_file(file_path.to_string())?;
    let graph_order = graph.get_num_vertices();
    let graph_size = graph.get_graph_size();
    let pop_size: usize = (graph.get_num_vertices() as f64 / 1.5).round() as usize;

    println!("graph_name,graph_order,graph_size,fitness_value,elapsed_time(microsecond)");

    let mut rdga = RomanDominationGA::new(graph, Some(pop_size));

    for _ in 0..trials {
        let start = Instant::now();
        let solution = rdga.run(
            GENERATIONS,
            MAX_STAGNANT,
            TOURNMENT_SIZE,
            CROSSOVER_PROBABILITY,
        );
        let end = Instant::now();
        let elapsed_time = end.duration_since(start);
        let elapsed_in_microseconds = elapsed_time.as_micros();

        println!(
            "{},{},{},{},{}",
            file_name,
            graph_order,
            graph_size,
            solution.fitness.unwrap_or_default(),
            elapsed_in_microseconds
        );
    }

    Ok(())
}
