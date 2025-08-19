use gpo::Individual;
use gpo::Population;
use rand::Rng;

#[derive(Clone)]
pub struct VectorIndividual {
    genes: Vec<f64>,
}

impl VectorIndividual {
    pub fn new(genes: Vec<f64>) -> Self {
        Self { genes }
    }
}

impl Individual for VectorIndividual {
    fn new_random() -> Self {
        let mut rng = rand::rng();
        let genes: Vec<f64> = (0..2) // 2-dimensional example
            .map(|_| rng.random_range(-10.0..10.0))
            .collect();
        Self::new(genes)
    }

    fn cost(&self) -> f64 {
        // Simple objective: minimize sum of squares
        self.genes.iter().map(|x| x * x).sum()
    }

    fn mutate(&self) -> Self {
        let mut rng = rand::rng();
        let mut new_genes = self.genes.clone();
        let idx = rng.random_range(0..new_genes.len());
        let delta: f64 = rng.random_range(-0.5..0.5);
        new_genes[idx] += delta;
        Self::new(new_genes)
    }

    fn crossover(&self, other: &Self) -> (Self, Self) {
        let mut rng = rand::rng();
        let point = rng.random_range(0..self.genes.len());
        let mut child1 = self.genes[..point].to_vec();
        child1.extend_from_slice(&other.genes[point..]);
        let mut child2 = other.genes[..point].to_vec();
        child2.extend_from_slice(&self.genes[point..]);
        (Self::new(child1), Self::new(child2))
    }
}

// Test GA
fn main() {
    let mut pop = Population::<VectorIndividual>::new_random(20);
    pop.evolve(500);

    let best = pop.best_individual();
    println!("Best individual: {:?}, fitness: {}", best.genes, best.cost());
}