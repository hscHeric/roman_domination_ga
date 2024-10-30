use genetic_algorithm::RomanDominationGA;
use graph::Graph;

mod genetic_algorithm;
mod graph;

use std::env;
use std::path::Path;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 7 {
        eprintln!("Uso: {} <file_path> <population_size> <max_generations> <max_stagnant> <tournament_size> <crossover_probability>", args[0]);
        exit(1);
    }

    let file_path = args[1].clone();
    let population_size: usize = args[2].parse().expect("Erro ao ler population_size");
    let max_generations: usize = args[3].parse().expect("Erro ao ler max_generations");
    let max_stagnant: usize = args[4].parse().expect("Erro ao ler max_stagnant");
    let tournament_size: usize = args[5].parse().expect("Erro ao ler tournament_size");
    let crossover_probability: f32 = args[6].parse().expect("Erro ao ler crossover_probability");

    let file_name = Path::new(&file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("arquivo");

    let graph = Graph::from_file(file_path.to_string())?;

    let mut rdga = RomanDominationGA::new(graph, Some(population_size));

    let best_solution = rdga.run(
        max_generations,
        max_stagnant,
        tournament_size,
        crossover_probability,
    );

    println!(
        "{},{},{}",
        file_name,
        best_solution.fitness.unwrap(),
        max_generations
    );

    Ok(())
}
