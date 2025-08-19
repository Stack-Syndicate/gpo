use rand::Rng;
use gpo::evolutionary::genetic_algorithm::{Individual, Population};

// Define an individual with one variable
#[derive(Clone)]
struct ScalarIndividual {
    x: f64,
    cached_fitness: f32,
}

impl Individual for ScalarIndividual {
    fn new_random() -> Self {
        let x = rand::rng().random_range(-10.0..10.0);
        let cached_fitness = (x * x) as f32;
        Self { x, cached_fitness }
    }

    fn mutate(&mut self) {
        let mut rng = rand::rng();
        self.x += rng.random_range(-1.0..1.0);
        self.cached_fitness = (self.x * self.x) as f32;
    }

    fn crossover(&self, other: Self) -> Vec<Self> {
        // Simple averaging crossover
        let child1 = ScalarIndividual {
            x: (self.x + other.x) / 2.0,
            cached_fitness: 0.0,
        };
        let child2 = ScalarIndividual {
            x: (self.x - other.x) / 2.0,
            cached_fitness: 0.0,
        };
        vec![child1, child2]
    }

    fn fitness(&self) -> f32 {
        self.cached_fitness
    }
}

fn main() {
    // Tournament selection function: pick the best of k random individuals
    let selection_fn = Box::new(|individuals: &Vec<ScalarIndividual>, rng: &mut rand::rngs::ThreadRng| {
        let k = 3;
        let mut best_index = rng.random_range(0..individuals.len());
        for _ in 1..k {
            let candidate = rng.random_range(0..individuals.len());
            if individuals[candidate].fitness() < individuals[best_index].fitness() {
                best_index = candidate;
            }
        }
        best_index
    });

    let mut population = Population::<ScalarIndividual>::new(5000, 0.1, selection_fn);

    population.run(1000); // Run 100 generations

    let best = population.get_best();
    println!("Best x: {}, fitness: {}", best.x, best.fitness());
}
