use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    vec,
};

use crate::{
    graph::{self, Graph},
    solution,
};

#[derive(Debug)]
struct Solution {
    labels: Vec<u8>,
    fitness: Option<usize>,
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

struct RomanDominationGA {
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

    pub fn generate_initial_population(&self) -> Vec<Solution> {
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
        let labels = self.graph.h1();
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

    pub fn evaluate_fitness(&mut self, solution: &mut Solution) {
        if solution.fitness.is_none() {
            solution.fitness = Some(solution.labels.iter().map(|&x| x as usize).sum());
        }
        solution.fitness.unwrap();
    }

    fn is_feasible(&self, solution: &Solution) -> bool {
        for vertex in 0..self.graph.get_num_vertices() {
            if solution.labels[vertex] == 0 {
                if !self
                    .graph
                    .get_neighbors(vertex)
                    .iter()
                    .any(|&neighbor| solution.labels[neighbor] == 2)
                {
                    return false;
                }
            }
        }
        true
    }

    fn make_feasible(&self, solution: &mut Solution) {
        for vertex in 0..self.graph.get_num_vertices() {
            if solution.labels[vertex] == 0 {
                if !self
                    .graph
                    .get_neighbors(vertex)
                    .iter()
                    .any(|&neighbor| solution.labels[neighbor] == 2)
                {
                    solution.labels[vertex] = 1;
                }
            }
        }
        solution.fitness = None; // Reseta o fitness na solução
    }
}
