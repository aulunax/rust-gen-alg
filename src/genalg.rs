use std::error::Error;

use rand::{Rng, seq::IndexedRandom};
use roulette_wheel::RouletteWheel;

use crate::individual::genetic::Genetic;

#[derive(Clone, Debug)]
pub struct FitnessIndiv<T: Genetic + Clone> {
    obj: T,
    fitness: f32,
}

impl<T: Genetic + Clone> FitnessIndiv<T> {
    pub fn fitness(&self) -> f32 {
        self.fitness
    }

    pub fn obj(&self) -> &T {
        &self.obj
    }

    pub fn new(obj: &T) -> Self {
        FitnessIndiv {
            obj: obj.clone(),
            fitness: obj.fitness(),
        }
    }

    pub fn into_tuple(&self) -> (f32, T) {
        (self.fitness, self.obj.clone())
    }
}

/// Genetic Algorithm struct
pub struct GenAlg<T: Genetic + Clone> {
    population_history: Vec<Vec<FitnessIndiv<T>>>,
    current_population: Vec<FitnessIndiv<T>>,
    current_generation: usize,
    best_individual: Option<FitnessIndiv<T>>,
}

impl<T: Genetic + Clone> GenAlg<T> {
    fn try_update_best_individual(&mut self) -> () {
        if let None = self.best_individual {
            self.best_individual = Some(self.current_population[0].clone());
            return;
        } else if let Some(v) = &self.best_individual {
            if v.fitness() < self.current_population[0].fitness() {
                self.best_individual = Some(self.current_population[0].clone());
            }
        }
    }

    fn validate_ga_input(&self, selection_rate: f32, mutation_rate: f32, elite_count: usize) -> () {
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
    }

    pub fn population_history(&self) -> &Vec<Vec<FitnessIndiv<T>>> {
        &self.population_history
    }

    /// Returns the best individual found by GA
    pub fn run_genetic_algorithm(
        &mut self,
        num_of_generations: usize,
        selection_rate: f32,
        mutation_rate: f32,
        elite_count: usize,
    ) -> Result<FitnessIndiv<T>, Box<dyn Error>> {
        self.validate_ga_input(selection_rate, mutation_rate, elite_count);

        let selected_count =
            (self.current_population.len() as f32 * selection_rate).floor() as usize;

        assert!(
            selected_count >= 2,
            "selection_rate too small. selection_rate should be large enough, to select at least 2 individuals."
        );

        let mut rng = rand::rng();

        let population_size = self.current_population.len();

        // sort population by fitness
        self.current_population
            .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());

        self.try_update_best_individual();

        for generation in 0..num_of_generations {
            let old_pop = self.current_population.clone();
            self.population_history.push(old_pop);

            // get top selected individuals
            //self.current_population.truncate(selected_count);

            // roulette wheel selection
            // let rw: RouletteWheel<T> = self
            //     .current_population
            //     .iter()
            //     .map(|indiv| indiv.into_tuple())
            //     .collect();

            // self.current_population = rw
            //     .into_iter()
            //     .map(|(f, ind)| FitnessIndiv {
            //         obj: ind,
            //         fitness: f,
            //     })
            //     .collect();

            self.current_population.truncate(selected_count);

            // crossover
            while self.current_population.len() != population_size {
                let parents = self.current_population[..selected_count]
                    .choose_multiple(&mut rng, 2)
                    .collect::<Vec<_>>();

                let child = parents[0].obj.crossover(&parents[1].obj);
                self.current_population.push(FitnessIndiv::new(&child));
            }

            // mutation, except in last generation
            if generation != num_of_generations - 1 {
                for indiv in self.current_population.iter_mut() {
                    if rng.random::<f32>() < mutation_rate {
                        indiv.obj.mutate();
                        indiv.fitness = indiv.obj.fitness();
                    }
                }
            }

            // sort new population by fitness
            self.current_population
                .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());

            // update best
            self.try_update_best_individual();

            self.current_generation += 1;
        }

        Ok(self.best_individual.as_ref().unwrap().clone())
    }

    pub fn new(population_size: usize, initial_population: Option<&Vec<T>>) -> Self {
        let mut start_population: Vec<FitnessIndiv<T>> = Vec::with_capacity(population_size);

        match initial_population {
            Some(init_pop) => {
                start_population = init_pop.into_iter().map(|a| FitnessIndiv::new(a)).collect();
            }
            None => start_population
                .extend((0..population_size).map(|_| FitnessIndiv::new(&T::generate()))),
        }

        Self {
            population_history: Vec::new(),
            current_population: start_population,
            current_generation: 0,
            best_individual: None,
        }
    }

    pub fn get_total_fitness(&self) -> f32 {
        self.current_population
            .iter()
            .map(|ind| ind.fitness())
            .sum()
    }

    pub fn calc_total_fitness(population: &Vec<FitnessIndiv<T>>) -> f32 {
        population.iter().map(|ind| ind.fitness()).sum()
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
            1000.0 / 2.0_f32.powf(((self.a - self.b).abs() as f32).sqrt())
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

    fn are_vals_in_range(vect: &Vec<FitnessIndiv<DummyGenetic>>) -> bool {
        vect.iter().all(|individual| {
            (0..MAX_RAND).contains(&individual.obj.a) && (0..MAX_RAND).contains(&individual.obj.b)
        })
    }

    fn are_vals_randomly_generated(vect: &Vec<FitnessIndiv<DummyGenetic>>) -> bool {
        !(vect
            .iter()
            .all(|x| x.obj.a == vect[0].obj.a && x.obj.b == vect[0].obj.b))
    }

    fn are_populations_same(
        vect: &Vec<FitnessIndiv<DummyGenetic>>,
        other: &Vec<FitnessIndiv<DummyGenetic>>,
    ) -> bool {
        vect.iter()
            .zip(other)
            .all(|(a, b)| a.obj == b.obj && a.fitness == b.fitness)
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

        assert_eq!(gen_alg.current_generation, NUM_GENS);
        assert!(
            are_vals_in_range(gen_vec),
            "One or more values are out of range!"
        );
    }

    #[test]
    #[should_panic]
    fn test_run_genetic_algorithm_selection_rate_low() {
        let mut gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);

        println!("{:?}", gen_alg.current_population);

        gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.001, 0.0, 0)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_run_genetic_algorithm_selection_rate_not_in_range() {
        let mut gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);

        println!("{:?}", gen_alg.current_population);

        gen_alg
            .run_genetic_algorithm(NUM_GENS, 2.0, 0.0, 0)
            .unwrap();
    }

    #[test]
    fn test_run_genetic_algorithm_selection_rate_in_range() {
        let mut gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);

        println!("{:?}", gen_alg.current_population);

        gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.1, 0.0, 0)
            .unwrap();
        gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.2, 0.0, 0)
            .unwrap();
        gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.5, 0.0, 0)
            .unwrap();
        gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.8, 0.0, 0)
            .unwrap();
        gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.9, 0.0, 0)
            .unwrap();
        gen_alg
            .run_genetic_algorithm(NUM_GENS, 1.0, 0.0, 0)
            .unwrap();
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
    fn test_run_genetic_algorithm_history() {
        let mut gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);

        println!("{:?}", gen_alg.current_population);
        gen_alg.run_genetic_algorithm(1, 0.5, 0.0, 0).unwrap();
        let second_pop = gen_alg.current_population.clone();
        gen_alg.run_genetic_algorithm(1, 0.5, 0.0, 0).unwrap();
        let third_pop = gen_alg.current_population.clone();
        gen_alg.run_genetic_algorithm(1, 0.5, 0.0, 0).unwrap();
        let fourth_pop = gen_alg.current_population.clone();
        gen_alg.run_genetic_algorithm(1, 0.5, 0.0, 0).unwrap();

        assert!(are_populations_same(
            &second_pop,
            &gen_alg.population_history()[1].clone()
        ));
        assert!(are_populations_same(
            &third_pop,
            &gen_alg.population_history()[2].clone()
        ));
        assert!(are_populations_same(
            &fourth_pop,
            &gen_alg.population_history()[3].clone()
        ));
    }

    #[test]
    #[ignore]
    fn test_run_genetic_algorithm_dummy_fitness_speed_bulk() {
        let start_timer = Instant::now();
        println!("Running fitness test {:?} times", SPEED_TEST_BULK_COUNT);

        for _ in 0..SPEED_TEST_BULK_COUNT {
            let mut gen_alg = GenAlg::<DummyGenetic>::new(FITNESS_TEST_POP_SIZE, None);

            gen_alg
                .run_genetic_algorithm(
                    FITNESS_TEST_NUM_GENS,
                    FITNESS_TEST_SELECTION_RATE,
                    FITNESS_TEST_MUTATION_RATE,
                    FITNESS_TEST_ELITE_COUNT,
                )
                .unwrap();
        }

        let duration = start_timer.elapsed();
        println!("Time elapsed: {:?}", duration);
    }
}
