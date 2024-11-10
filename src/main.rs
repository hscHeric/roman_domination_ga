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
        eprintln!(
            "Uso: {} <file_path> <trials> [max_stagnant] [generations] [tournament_size] [crossover_prob] [pop_size]",
            args[0]
        );
        exit(1);
    }

    // Parâmetros obrigatórios
    let file_path = args[1].clone();
    let trials: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Erro: 'trials' deve ser um número inteiro positivo.");
            exit(1);
        }
    };

    // Valores default
    const DEFAULT_MAX_STAGNANT: usize = 100;
    const DEFAULT_GENERATIONS: usize = 1000;
    const DEFAULT_TOURNAMENT_SIZE: usize = 2;
    const DEFAULT_CROSSOVER_PROBABILITY: f32 = 0.9;

    // Parâmetros opcionais
    let max_stagnant = if args.len() > 3 {
        match args[3].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Erro: 'max_stagnant' deve ser um número inteiro positivo.");
                exit(1);
            }
        }
    } else {
        DEFAULT_MAX_STAGNANT
    };

    let generations = if args.len() > 4 {
        match args[4].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Erro: 'generations' deve ser um número inteiro positivo.");
                exit(1);
            }
        }
    } else {
        DEFAULT_GENERATIONS
    };

    let tournament_size = if args.len() > 5 {
        match args[5].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Erro: 'tournament_size' deve ser um número inteiro positivo.");
                exit(1);
            }
        }
    } else {
        DEFAULT_TOURNAMENT_SIZE
    };

    let crossover_probability = if args.len() > 6 {
        match args[6].parse() {
            Ok(n) => {
                if n < 0.0 || n > 1.0 {
                    eprintln!("Erro: 'crossover_probability' deve estar entre 0 e 1.");
                    exit(1);
                }
                n
            }
            Err(_) => {
                eprintln!("Erro: 'crossover_probability' deve ser um número entre 0 e 1.");
                exit(1);
            }
        }
    } else {
        DEFAULT_CROSSOVER_PROBABILITY
    };

    let file_name = Path::new(&file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("arquivo");

    let graph = Graph::from_file(file_path.to_string())?;
    let graph_order = graph.get_num_vertices();
    let graph_size = graph.get_graph_size();

    // pop_size agora pode ser passado como parâmetro opcional
    let pop_size = if args.len() > 7 {
        match args[7].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Erro: 'pop_size' deve ser um número inteiro positivo.");
                exit(1);
            }
        }
    } else {
        (graph.get_num_vertices() as f64 / 1.5).round() as usize
    };

    println!("graph_name,graph_order,graph_size,fitness_value,elapsed_time(microsecond)");
    let mut rdga = RomanDominationGA::new(graph, Some(pop_size));

    for _ in 0..trials {
        let start = Instant::now();
        let solution = rdga.run(
            generations,
            max_stagnant,
            tournament_size,
            crossover_probability,
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
