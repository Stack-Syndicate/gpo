pub trait Individual: Clone {
    fn new_random() -> Self; 
    fn cost(&self) -> f64;
    fn mutate(&self) -> Self;
    fn crossover(&self, other: &Self) -> (Self, Self);
}

pub struct Population<I: Individual> {
    individuals: Vec<I>,
}
impl<I: Individual> Population<I> {
    pub fn new_random(pop_size: usize) -> Self {
        let mut individuals = Vec::new();
        for i in 0..pop_size {
            individuals.push(I::new_random());
        }
        Self {individuals}
    }
    pub fn evolve(&mut self, generations: usize) {
        for _ in 0..generations {
            // Evaluate fitness
            self.individuals.sort_by(|a, b| a.cost().partial_cmp(&b.cost()).unwrap());

            // Selection, crossover, mutation
            let mut next_gen = vec![];
            while next_gen.len() < self.individuals.len() {
                // Selection example: tournament
                let parent1 = self.tournament_select(3);
                let parent2 = self.tournament_select(3);

                let (child1, child2) = parent1.crossover(parent2);
                next_gen.push(child1.mutate());
                next_gen.push(child2.mutate());
            }
            self.individuals = next_gen;
        }
    }
    pub fn best_individual(&self) -> &I {
        self.individuals
            .iter()
            .min_by(|a, b| a.cost().partial_cmp(&b.cost()).unwrap())
            .unwrap()
    }
    fn tournament_select(&self, k: usize) -> &I {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut best = &self.individuals[rng.random_range(0..self.individuals.len())];
        for _ in 1..k {
            let candidate = &self.individuals[rng.random_range(0..self.individuals.len())];
            if candidate.cost() < best.cost() {
                best = candidate;
            }
        }
        best
    }
}