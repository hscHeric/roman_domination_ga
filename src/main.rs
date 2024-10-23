use std::env;

use graph::Graph;

mod genetic_algorithm;
pub mod graph;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <caminho do arquivo>", args[0]);
        std::process::exit(1); // Sai com erro
    }

    let file_path = &args[1];

    match Graph::from_file(file_path.to_string()) {
        Ok(g) => {
            print!("Grafo carregado com sucesso.");

            g.display();
        }

        Err(e) => {
            eprintln!("Erro ao carregar o grafo: {}", e);
            std::process::exit(1); // Sai com erro
        }
    }
}
