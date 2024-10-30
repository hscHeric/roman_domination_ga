use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    vec,
};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::graph::Graph;

pub struct Solution {
    labels: Vec<u8>,
    pub fitness: Option<usize>,
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        Self {
            labels: self.labels.clone(),
            fitness: self.fitness, // Option implementa a trait de copy
        }
    }
}

impl Solution {
    fn new(labels: Vec<u8>, fitness: Option<usize>) -> Self {
        Solution { labels, fitness }
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.fitness, other.fitness) {
            (Some(f1), Some(f2)) => Some(f1.cmp(&f2)),
            (None, Some(_)) => Some(Ordering::Less), // Considerar None como menor
            (Some(_), None) => Some(Ordering::Greater), // Considerar Some como maior
            (None, None) => Some(Ordering::Equal),   // Ambos são None
        }
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap() // Aqui não precisa do unwrap_or
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Solution {}

pub struct RomanDominationGA {
    graph: Graph,
    population_size: usize,
}

impl RomanDominationGA {
    pub fn new(graph: Graph, population_size: Option<usize>) -> Self {
        let population_size = population_size.unwrap_or(graph.get_num_vertices() / 2);
        RomanDominationGA {
            graph,
            population_size,
        }
    }

    fn generate_initial_population(&self) -> Vec<Solution> {
        let mut population = vec![];

        let h2_solution = self.generate_h2_solution();
        population.push(h2_solution);

        while population.len() < self.population_size {
            let h1_solution = self.generate_h1_solution();
            population.push(h1_solution);
        }

        population
    }

    fn generate_h2_solution(&self) -> Solution {
        let labels = self.graph.h2();
        let fitness = if labels.is_empty() {
            None
        } else {
            Some(labels.iter().map(|&x| x as usize).sum())
        };

        Solution::new(labels, fitness)
    }

    fn generate_h1_solution(&self) -> Solution {
        let labels = self.graph.h1();
        let fitness = if labels.is_empty() {
            None
        } else {
            Some(labels.iter().map(|&x| x as usize).sum())
        };

        Solution::new(labels, fitness)
    }

    fn evaluate_fitness(&self, solution: &mut Solution) -> usize {
        if solution.fitness.is_none() {
            solution.fitness = Some(solution.labels.iter().map(|&x| x as usize).sum());
        }
        solution.fitness.unwrap()
    }

    fn is_feasible(&self, solution: &Solution) -> bool {
        for vertex in 0..self.graph.get_num_vertices() {
            if solution.labels[vertex] == 0
                && !self
                    .graph
                    .get_neighbors(vertex)
                    .iter()
                    .any(|&neighbor| solution.labels[neighbor] == 2)
            {
                return false;
            }
        }
        true
    }

    fn make_feasible(&self, solution: &mut Solution) {
        for vertex in 0..self.graph.get_num_vertices() {
            if solution.labels[vertex] == 0
                && !self
                    .graph
                    .get_neighbors(vertex)
                    .iter()
                    .any(|&neighbor| solution.labels[neighbor] == 2)
            {
                solution.labels[vertex] = 1;
            }
        }
        solution.fitness = None; // Reseta o fitness na solução
    }

    fn tournament_selection(
        &mut self,
        tournament_size: usize,
        population: &mut Vec<Solution>,
    ) -> Vec<Solution> {
        let mut selected = Vec::with_capacity(population.len());
        let mut rng = thread_rng();

        // Primeiro, garantimos que todas as soluções têm fitness calculado
        for solution in population.iter_mut() {
            if solution.fitness.is_none() {
                self.evaluate_fitness(solution);
            }
        }

        while selected.len() < population.len() {
            // Seleciona índices aleatórios para o torneio
            let tournament_indices: Vec<usize> = (0..population.len())
                .collect::<Vec<_>>()
                .choose_multiple(&mut rng, tournament_size)
                .cloned()
                .collect();

            // Encontra o melhor indivíduo do torneio
            let winner_index = tournament_indices
                .iter()
                .min_by_key(|&&idx| population[idx].fitness.unwrap())
                .unwrap();

            // Clone apenas o vencedor
            selected.push(population[*winner_index].clone());
        }

        selected
    }

    fn crossover(&self, parent_a: &Solution, parent_b: &Solution) -> Solution {
        let mut rng = thread_rng();
        let (r1, r2) = {
            let mut indices: Vec<usize> = (0..self.graph.get_num_vertices()).collect();
            indices.shuffle(&mut rng);
            (indices[0], indices[1])
        };

        let (r1, r2) = (r1.min(r2), r1.max(r2));
        let mut child_labels = parent_a.labels.clone();
        child_labels[r1..r2].copy_from_slice(&parent_b.labels[r1..r2]);
        let mut child = Solution::new(child_labels, None);
        self.evaluate_fitness(&mut child);

        if !self.is_feasible(&child) {
            self.make_feasible(&mut child);
        }

        child
    }
    pub fn run(
        &mut self,
        max_generations: usize,
        max_stagnant: usize,
        tournament_size: usize,
        crossover_probability: f32,
    ) -> Solution {
        let mut population: Vec<Solution> = self.generate_initial_population();

        for solution in &mut population {
            self.evaluate_fitness(solution);
        }

        if population.iter().any(|s| s.fitness.is_none()) {
            eprintln!("Erro: Existem soluções sem fitness calculado na população inicial.");
            return Solution::new(vec![], None); // Retorne uma solução vazia ou trate conforme necessário
        }

        // Encontre a melhor solução inicial
        let mut best_solution = population
            .iter()
            .min_by_key(|s| s.fitness.unwrap())
            .unwrap()
            .clone();

        let mut stagnant_generations = 0;

        for _ in 0..max_generations {
            //   println!("{} de {}", i, max_generations);
            if stagnant_generations >= max_stagnant {
                break;
            }

            let intermediate_pop = self.tournament_selection(tournament_size, &mut population);
            let mut new_pop = Vec::with_capacity(population.len());

            for i in (0..intermediate_pop.len()).step_by(2) {
                // Avalia o fitness antes de aplicar crossover
                if i < intermediate_pop.len() {
                    let mut solution_clone = intermediate_pop[i].clone();
                    self.evaluate_fitness(&mut solution_clone);
                    new_pop.push(solution_clone);
                }

                if i + 1 < intermediate_pop.len() {
                    let mut solution_clone_2 = intermediate_pop[i + 1].clone();
                    self.evaluate_fitness(&mut solution_clone_2);
                    new_pop.push(solution_clone_2);
                }

                // Realiza crossover com base na probabilidade
                if rand::random::<f32>() < crossover_probability && i + 1 < intermediate_pop.len() {
                    let mut child = self.crossover(&intermediate_pop[i], &intermediate_pop[i + 1]);
                    self.evaluate_fitness(&mut child);
                    new_pop.push(child);
                }
            }

            population = new_pop;

            // Avalia o fitness de todas as soluções na nova população
            for solution in &mut population {
                self.evaluate_fitness(solution);
            }

            // Verifica se todas as soluções têm fitness calculado
            if population.iter().any(|s| s.fitness.is_none()) {
                eprintln!("Erro: Existem soluções sem fitness calculado na nova população.");
                return Solution::new(vec![], None); // Retorne uma solução vazia ou trate conforme necessário
            }

            // Encontre a melhor solução na nova população
            let current_best = population
                .iter()
                .min_by_key(|s| s.fitness.unwrap())
                .unwrap()
                .clone();

            // Atualiza a melhor solução encontrada e verifica estagnação
            if current_best.fitness < best_solution.fitness {
                best_solution = current_best;
                stagnant_generations = 0; // Reseta estagnação
            } else {
                stagnant_generations += 1;
            }
        }

        best_solution
    }
}
