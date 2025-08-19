pub trait Individual: Clone {
    fn new_random() -> Self;
    fn mutate(&mut self);
    fn crossover(&self, other: Self) -> Vec<Self>;
    fn fitness(&self) -> f32;
}

pub struct Population<I: Individual> {
    individuals: Vec<I>,
    elitism_fraction: f32,
    selection_fn: Box<dyn Fn(&Vec<I>, &mut rand::rngs::ThreadRng) -> usize>,
}

impl<I: Individual> Population<I> {
    pub fn new(
        size: usize,
        elitism_fraction: f32,
        selection_fn: Box<dyn Fn(&Vec<I>, &mut rand::rngs::ThreadRng) -> usize>,
    ) -> Self {
        let mut individuals = Vec::with_capacity(size);
        for _ in 0..size {
            individuals.push(I::new_random());
        }
        Self {
            individuals,
            elitism_fraction,
            selection_fn,
        }
    }

    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    pub fn get(&self, index: usize) -> &I {
        &self.individuals[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut I {
        &mut self.individuals[index]
    }

    pub fn sort(&mut self) {
        // Sort ascending if smaller fitness is better
        self.individuals
            .sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());
    }

    fn run_generation(&mut self) {
        self.sort();
        let mut new_individuals: Vec<I> = Vec::with_capacity(self.individuals.len());

        // Elitism
        let num_elite = (self.individuals.len() as f32 * self.elitism_fraction) as usize;
        for i in 0..num_elite {
            new_individuals.push(self.individuals[i].clone());
        }

        let mut rng = rand::rng();

        // Crossover and Mutation
        while new_individuals.len() < self.individuals.len() {
            let parent1_index = (self.selection_fn)(&self.individuals, &mut rng);
            let parent2_index = (self.selection_fn)(&self.individuals, &mut rng);

            let parent1 = self.individuals[parent1_index].clone();
            let parent2 = self.individuals[parent2_index].clone();

            let mut children = parent1.crossover(parent2);

            for child in children.iter_mut() {
                child.mutate();
            }

            for child in children {
                if new_individuals.len() < self.individuals.len() {
                    new_individuals.push(child);
                } else {
                    break;
                }
            }
        }

        self.individuals = new_individuals;
    }

    pub fn run(&mut self, generations: usize) {
        for _ in 0..generations {
            self.run_generation();
        }
    }

    pub fn get_best(&mut self) -> &I {
        self.sort();
        &self.individuals[0]
    }
}
