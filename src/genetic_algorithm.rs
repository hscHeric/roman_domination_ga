use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

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
