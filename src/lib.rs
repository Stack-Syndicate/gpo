use rand::Rng;

pub struct PopulationBuilder {
    dimensions: Vec<(&'static str, f64, f64)>,
    swarm_size: usize,
    inertia: f64,
    cognitive: f64,
    social: f64,
    objective: Option<Box<dyn Fn(&[f64]) -> f64>>,
    search: Vec<Box<dyn Fn(f64) -> f64>>,
}

impl PopulationBuilder {
    pub fn new() -> Self {
        Self {
            dimensions: vec![("x", -10.0, 10.0)],
            swarm_size: 100,
            inertia: 0.0,
            cognitive: 0.0,
            social: 0.0,
            objective: Some(Box::new(|p: &[f64]| -> f64 {
                let x = p[0];
                x * x
            })),
            search: vec![]
        }
    }
    pub fn dimensions(mut self, dims: &[(&'static str, f64, f64)]) -> Self {
        self.dimensions = dims.to_vec();
        self
    }

    pub fn swarm_size(mut self, size: usize) -> Self {
        self.swarm_size = size;
        self
    }

    pub fn inertia(mut self, inertia: f64) -> Self {
        self.inertia = inertia;
        self
    }

    pub fn cognitive(mut self, c: f64) -> Self {
        self.cognitive = c;
        self
    }

    pub fn social(mut self, s: f64) -> Self {
        self.social = s;
        self
    }

    pub fn objective<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(&[f64]) -> f64,
    {
        self.objective = Some(Box::new(f));
        self
    }
    pub fn search<F>(mut self, funcs: Vec<F>) -> Self
    where
        F: 'static + Fn(f64) -> f64,
    {
        self.search.clear();
        for func in funcs {
            self.search.push(Box::new(func));
        }
        self
    }
    pub fn build(self) -> Population {
        let objective = self.objective.expect("Objective function must be set");
        Population::new(
            self.dimensions,
            self.swarm_size,
            self.inertia,
            self.cognitive,
            self.social,
            objective,
            self.search
        )
    }
}


pub struct Population {
    dimensions: Vec<(&'static str, f64, f64)>,
    swarm_size: usize,
    inertia: f64,
    cognitive: f64,
    social: f64,
    objective: Box<dyn Fn(&[f64]) -> f64>,
    particles: Vec<Particle>,
    search: Vec<Box<dyn Fn(f64) -> f64>>
}
impl Population {
    pub fn init() -> PopulationBuilder {
        PopulationBuilder::new()
    }
    pub fn new(
        dimensions: Vec<(&'static str, f64, f64)>,
        swarm_size: usize,
        inertia: f64,
        cognitive: f64,
        social: f64,
        objective: Box<dyn Fn(&[f64]) -> f64>,
        search: Vec<Box<dyn Fn(f64) -> f64>>
    ) -> Self {
        let particles = (0..swarm_size)
            .map(|_| Particle::new(dimensions.clone()))
            .collect();
        Self {
            dimensions,
            swarm_size,
            inertia,
            cognitive,
            social,
            objective,
            particles,
            search
        }
    }

    pub fn run(&mut self, iterations: usize) -> Vec<f64> {
        let mut global_best_position: Vec<f64> = vec![];
        let mut global_best_fitness = f64::MAX;

        for _ in 0..iterations {
            for particle in &mut self.particles {
                let fitness = (self.objective)(&particle.position);

                if fitness < particle.best_fitness {
                    particle.best_fitness = fitness;
                    particle.best_position = particle.position.clone();
                }

                if fitness < global_best_fitness {
                    global_best_fitness = fitness;
                    global_best_position = particle.position.clone();
                }
            }

            for particle in &mut self.particles {
                for i in 0..particle.position.len() {
                    let r1: f64 = rand::rng().random();
                    let r2: f64 = rand::rng().random();
                    
                    let mut search_component = 0.0;
                    if self.search.len() == particle.position.len() {
                        search_component = (self.search[i])(particle.position[i]);
                    }
                    let cognitive_component =
                        self.cognitive * r1 * (particle.best_position[i] - particle.position[i]);
                    let social_component =
                        self.social * r2 * (global_best_position[i] - particle.position[i]);

                    particle.velocity[i] = self.inertia * particle.velocity[i]
                        + cognitive_component
                        + social_component
                        + search_component;
                    particle.position[i] += particle.velocity[i];

                    // Apply dimension constraints
                    let (_, min, max) = self.dimensions[i];
                    if particle.position[i] < min {
                        particle.position[i] = min;
                    } else if particle.position[i] > max {
                        particle.position[i] = max;
                    }
                }
            }
        }
        global_best_position
    }
}

pub struct Particle {
    position: Vec<f64>,
    velocity: Vec<f64>,
    best_position: Vec<f64>,
    best_fitness: f64,
}
impl Particle {
    pub fn new(dimensions: Vec<(&'static str, f64, f64)>) -> Self {
        let position: Vec<f64> = dimensions
            .iter()
            .map(|(_, min, max)| rand::rng().random_range(*min..*max))
            .collect();
        let velocity = dimensions
            .iter()
            .map(|(_, _, _)| rand::rng().random_range(-1.0..1.0))
            .collect();
        let best_position = position.clone();
        let best_fitness = f64::MAX;
        Particle {
            position,
            velocity,
            best_position,
            best_fitness,
        }
    }
}