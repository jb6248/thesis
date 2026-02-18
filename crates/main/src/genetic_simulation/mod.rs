mod grammar_derivation_genome;

use rand::{Rng, RngExt};
use rayon::prelude::*;

pub trait Genome {
    type Config;
    fn generate(config: &Self::Config, rng: &mut impl Rng) -> Self;
    fn mutate(&mut self, config: &Self::Config, rng: &mut impl Rng);
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self;
    fn fitness(&self) -> f64;
}

pub struct Simulation<G>
where
    G: Genome,
{
    population: Vec<G>,
    config: G::Config,
    p_crossover: f64,
    p_mutation: f64,
}

#[derive(Debug)]
pub struct SimulationMetrics<G> {
    pub average_fitness: f64,
    pub best_fitness: f64,
    /// The best genomes in the current population ordered from best to worst.
    /// Test parameters can be adjusted to return more than one best genome.
    pub best_genomes: Vec<(G, f64)>,
}

impl<G> SimulationMetrics<G> {
    pub fn summary(&self) -> String {
        format!(
            "Best Fitness: {:.4}, Average Fitness: {:.4}",
            self.best_fitness, self.average_fitness
        )
    }
}

impl<C, G: Genome<Config = C> + Clone + ?Sized + Sync> Simulation<G> {
    pub fn new(
        population_size: usize,
        config: C,
        p_crossover: f64,
        p_mutation: f64,
    ) -> Self {
        if population_size == 0 {
            panic!("Population size must be greater than 0");
        }
        let mut rng = rand::rng();
        let population = (0..population_size)
            .map(|_| G::generate(&config, &mut rng))
            .collect();
        Self {
            population,
            config,
            p_crossover,
            p_mutation,
        }
    }

    pub fn step(&mut self) {
        let fitnesses: Vec<_> = self
            .population
            .par_iter()
            .map(|genome| {
                let fitness = genome.fitness();
                (genome, fitness)
            })
            .collect();
        // normalize fitness scores
        let fitnesses: Vec<_> = {
            let (fit_min, fit_max) = fitnesses
                    .iter()
                    .map(|(_, fit)| *fit)
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), fit| {
                        (min.min(fit), max.max(fit))
                    });
            let fitness_range = fit_max - fit_min;
            fitnesses
                .into_iter()
                .map(|(genome, fit)| (genome, if fitness_range == 0.0 {
                    1.0 / self.population.len() as f64
                } else {
                    (fit - fit_min) / fitness_range
                }))
                .collect()
        };
        let mut rng = rand::rng();
        let mut sample = || -> &G {
            let p = rng.random::<f64>();
            let mut cumulative = 0.0;
            for (genome, fit) in fitnesses.iter() {
                if p < cumulative + *fit {
                    return *genome;
                }
                cumulative += *fit;
            }
            fitnesses.last().unwrap().0
        };

        let mut new_population = Vec::new();
        let mut rng = rand::rng();
        while new_population.len() < self.population.len() {
            let parent1 = sample();
            let parent2 = sample();

            let mut child = if rng.random::<f64>() < self.p_crossover {
                parent1.crossover(&parent2, &mut rng)
            } else {
                if rng.random::<f64>() < 0.5 {
                    parent1.clone()
                } else {
                    parent2.clone()
                }
            };

            if rng.random::<f64>() < self.p_mutation {
                child.mutate(&self.config, &mut rng);
            }

            let x = rand::random::<f64>();

            new_population.push(child);
        }

        self.population = new_population;
    }

    /// Calculate metrics for the current population.
    /// Pick how many of the best genomes to return.
    pub fn calculate_metrics(&self, top_n: usize) -> SimulationMetrics<G> {
        let mut fitnesses: Vec<(&G, f64)> = self
            .population
            .iter()
            .map(|genome| (genome, genome.fitness()))
            .collect();
        let best_fitness = fitnesses
            .iter()
            .map(|(_g, fit)| *fit)
            .fold(f64::NEG_INFINITY, |a, b| a.max(b));
        let average_fitness = fitnesses.iter().map(|(_g, fit)| fit).sum::<f64>() / fitnesses.len() as f64;

        if top_n > 0 {
            // sort by fitness descending
            fitnesses.sort_by(|(_g1, a), (_g2, b)| b.partial_cmp(&a).unwrap());
        }
        let best_candidates = fitnesses.into_iter().take(top_n).map(|(g, fit)| (g.clone(), fit)).collect::<Vec<_>>();
        SimulationMetrics {
            best_genomes: best_candidates,
            average_fitness,
            best_fitness,
        }
    }
}
