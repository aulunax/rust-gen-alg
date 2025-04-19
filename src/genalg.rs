use std::error::Error;

use rand::{Rng, rng, seq::IndexedRandom};

use crate::individual::genetic::Genetic;

pub struct GenAlg<T: Genetic + Clone> {
    current_population: Vec<T>,
    current_generation: usize,
}

impl<T: Genetic + Clone> GenAlg<T> {
    pub fn run_genetic_algorithm(
        &mut self,
        num_of_generations: usize,
        selection_rate: f32,
        mutation_rate: f32,
        elite_count: usize,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        assert!(
            elite_count <= self.current_population.len(),
            "elite_count cannot be greater than population size"
        );
        assert!(
            (0.0..=1.0).contains(&mutation_rate),
            "mutation_rate must be in [0.0, 1.0]"
        );
        assert!(
            (0.0..=1.0).contains(&selection_rate),
            "selection_rate must be in [0.0, 1.0]"
        );

        let mut rng = rand::rng();

        // sort population by fitness
        self.current_population
            .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());

        for generation in 0..num_of_generations {
            // put this in history maybe?
            let old_pop = self.current_population.clone();
            let mut new_pop: Vec<T> = Vec::with_capacity(self.current_population.len());

            // get top selected individuals
            new_pop
                .extend_from_slice(&self.current_population[..self.current_population.len() / 2]);

            // crossover
            while new_pop.len() != self.current_population.len() {
                let parents = self.current_population[..self.current_population.len() / 2]
                    .choose_multiple(&mut rng, 2)
                    .collect::<Vec<_>>();

                let child = parents[0].crossover(parents[1]);
                new_pop.push(child);
            }

            // mutation, except in last generation
            if generation != num_of_generations - 1 {
                for indiv in new_pop.iter_mut() {
                    if rng.random::<f32>() < mutation_rate {
                        indiv.mutate();
                    }
                }
            }

            // sort new population by fitness
            new_pop.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());

            // replace old population
            self.current_population = new_pop;

            self.current_generation += 1;
        }

        Ok(self.current_population.clone())
    }

    pub fn new(population_size: usize, initial_population: Option<&Vec<T>>) -> Self {
        let mut start_population: Vec<T> = Vec::with_capacity(population_size);

        match initial_population {
            Some(pop) => start_population = pop.clone(),
            None => start_population.extend((0..population_size).map(|_| T::generate())),
        }

        Self {
            current_population: start_population,
            current_generation: 0,
        }
    }

    pub fn get_total_fitness(&self) -> f32 {
        self.current_population
            .iter()
            .map(|ind| ind.fitness())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use rand::Rng;

    const MAX_RAND: i32 = 1000;

    #[derive(Clone, Debug, PartialEq)]
    struct DummyGenetic {
        a: i32,
        b: i32,
    }

    impl Genetic for DummyGenetic {
        fn generate() -> Self {
            let mut rng = rand::rng();
            let a = rng.random_range(0..MAX_RAND);
            let b = rng.random_range(0..MAX_RAND);

            DummyGenetic { a: a, b: b }
        }

        fn fitness(&self) -> f32 {
            -((self.a - self.b).abs()) as f32
        }

        fn crossover(&self, other: &Self) -> Self {
            let mut rng = rand::rng();
            if rng.random_bool(0.5) {
                DummyGenetic {
                    a: self.a,
                    b: other.b,
                }
            } else {
                DummyGenetic {
                    a: other.a,
                    b: self.b,
                }
            }
        }

        fn mutate(&mut self) -> () {
            let mut rng = rand::rng();
            match rng.random_bool(0.5) {
                true => self.a = rng.random_range(0..MAX_RAND),
                false => self.b = rng.random_range(0..MAX_RAND),
            }
        }
    }

    const POP_SIZE: usize = 100;
    const NUM_GENS: usize = 100;

    // Do not change these values for the fitness test
    // These values are set this way, so that
    // both correctness of the algorithm
    // and speed of the algorithm can be always checked
    const FITNESS_TEST_POP_SIZE: usize = 100;
    const FITNESS_TEST_NUM_GENS: usize = 100;
    const FITNESS_TEST_SELECTION_RATE: f32 = 0.5;
    const FITNESS_TEST_MUTATION_RATE: f32 = 0.05;
    const FITNESS_TEST_ELITE_COUNT: usize = 0;
    const EXPECTED_DUMMY_TOTAL_FITNESS_IMPROVEMENT_FACTOR: f32 = 2.0;

    const SPEED_TEST_BULK_COUNT: usize = 1000;

    fn are_vals_in_range(vect: &Vec<DummyGenetic>) -> bool {
        vect.iter().all(|individual| {
            (0..MAX_RAND).contains(&individual.a) && (0..MAX_RAND).contains(&individual.b)
        })
    }

    fn are_vals_randomly_generated(vect: &Vec<DummyGenetic>) -> bool {
        !(vect.iter().all(|x| x.a == vect[0].a && x.b == vect[0].b))
    }

    #[test]
    fn test_gen_alg_creation() {
        let gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);
        let gen_vec = &gen_alg.current_population;

        println!("{:?}", gen_vec);

        assert_eq!(gen_alg.current_population.len(), POP_SIZE);
        assert!(
            are_vals_in_range(gen_vec),
            "One or more values are out of range!"
        );
        assert!(
            are_vals_randomly_generated(gen_vec),
            "All elements in vector are equal, rng doesn't work"
        );
    }

    #[test]
    fn test_gen_alg_creation_init_pop() {
        let init_pop: Vec<DummyGenetic> = (0..POP_SIZE).map(|_| DummyGenetic::generate()).collect();

        let gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, Some(&init_pop));
        let gen_vec = &gen_alg.current_population;

        println!("{:?}", gen_vec);

        assert_eq!(gen_alg.current_population.len(), POP_SIZE);
        assert!(
            are_vals_in_range(gen_vec),
            "One or more values are out of range!"
        );
        assert!(
            are_vals_randomly_generated(gen_vec),
            "All elements in vector are equal, rng doesn't work"
        );
    }

    #[test]
    fn test_run_genetic_algorithm() {
        let mut gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);

        println!("{:?}", gen_alg.current_population);

        let result = gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.5, 0.05, 0)
            .unwrap();
        let gen_vec = &gen_alg.current_population;

        println!("{:?}", result);

        assert_eq!(result.len(), POP_SIZE);
        assert_eq!(gen_alg.current_generation, NUM_GENS);
        assert!(
            are_vals_in_range(&result),
            "One or more values are out of range!"
        );
        assert_eq!(&result, gen_vec);
    }

    #[test]
    fn test_run_genetic_algorithm_dummy_fitness() {
        let mut gen_alg = GenAlg::<DummyGenetic>::new(FITNESS_TEST_POP_SIZE, None);
        let starting_fitness = gen_alg.get_total_fitness();

        println!("{:?}", gen_alg.current_population);

        gen_alg
            .run_genetic_algorithm(
                FITNESS_TEST_NUM_GENS,
                FITNESS_TEST_SELECTION_RATE,
                FITNESS_TEST_MUTATION_RATE,
                FITNESS_TEST_ELITE_COUNT,
            )
            .unwrap();

        println!("{:?}", gen_alg.current_population);

        let final_fitness = gen_alg.get_total_fitness();
        println!("Starting fitness: {:?}", starting_fitness);
        println!("Final fitness: {:?}", final_fitness);

        assert!(
            final_fitness > starting_fitness / EXPECTED_DUMMY_TOTAL_FITNESS_IMPROVEMENT_FACTOR,
            "Total fitness largrly outside expectations. {:?} \nStarting fitness: {}, Final fitness: {}",
            gen_alg.current_population,
            starting_fitness,
            final_fitness
        );
    }

    #[test]
    #[ignore]
    fn test_run_genetic_algorithm_dummy_fitness_speed_bulk() {
        let start_timer = Instant::now();
        println!("Running fitness test {:?} times", SPEED_TEST_BULK_COUNT);

        for _ in 0..SPEED_TEST_BULK_COUNT {
            let mut gen_alg = GenAlg::<DummyGenetic>::new(FITNESS_TEST_POP_SIZE, None);
            let starting_fitness = gen_alg.get_total_fitness();

            gen_alg
                .run_genetic_algorithm(
                    FITNESS_TEST_NUM_GENS,
                    FITNESS_TEST_SELECTION_RATE,
                    FITNESS_TEST_MUTATION_RATE,
                    FITNESS_TEST_ELITE_COUNT,
                )
                .unwrap();

            let final_fitness = gen_alg.get_total_fitness();

            assert!(
                final_fitness > starting_fitness / EXPECTED_DUMMY_TOTAL_FITNESS_IMPROVEMENT_FACTOR,
                "Total fitness largrly outside expectations. {:?} \nStarting fitness: {}, Final fitness: {}",
                gen_alg.current_population,
                starting_fitness,
                final_fitness
            );
        }

        let duration = start_timer.elapsed();
        println!("Time elapsed: {:?}", duration);
    }
}
