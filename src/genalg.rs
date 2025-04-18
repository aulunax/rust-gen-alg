use std::error::Error;

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
        mutatation_rate: f32,
        elite_count: usize
    ) -> Result<Vec<T>, Box<dyn Error>> {

        if elite_count > self.current_population.len() {
            panic!("Number of elite individuals can't be higher than total number of individuals in a generation");
        }

        for _ in 0..num_of_generations {
            //
            // Actual code goes here
            //

            self.current_generation += 1;
        }

        Ok(self.current_population.clone())
    }

    pub fn new(
        population_size: usize,
        initial_population: Option<&Vec<T>>,
    ) -> Self {
        let mut start_population: Vec<T> = Vec::with_capacity(population_size);

        match initial_population {
            Some(pop) => start_population = pop.clone(),
            None => start_population.extend((0..population_size).map(|_| T::generate())),
        }

        Self { current_population: start_population, current_generation: 0 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const MAX_RAND: i32 = 1000;

    #[derive(Clone, Debug, PartialEq)]
    struct DummyGenetic {
        value: i32,
    }

    impl Genetic for DummyGenetic {
        fn generate() -> Self {
            let mut rng = rand::rng();
            let random_value = rng.random_range(0..MAX_RAND);

            DummyGenetic { value: random_value }
        }

        fn fitness(&self) -> f32 {
            todo!();
        }
        
        fn crossover(&self, other: Self) -> Self {
            todo!()
        }
        
        fn mutate(&self) -> () {
            todo!()
        }

    }

    const POP_SIZE: usize = 20;
    const NUM_GENS: usize = 10;

    #[test]
    fn test_gen_alg_creation() {
        let gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);
        let gen_vec = &gen_alg.current_population;

        println!("{:?}", gen_vec);

        assert_eq!(gen_alg.current_population.len(), POP_SIZE);
        assert!(
            gen_vec.iter().all(|individual| (0..MAX_RAND).contains(&individual.value)),
            "One or more values are out of range!"
        );
        assert!(!(gen_vec.iter().all(|x| x.value == gen_vec[0].value)), 
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
            gen_vec.iter().all(|individual| (0..MAX_RAND).contains(&individual.value)),
            "One or more values are out of range!"
        );
        assert!(!(gen_vec.iter().all(|x| x.value == gen_vec[0].value)), 
            "All elements in vector are equal, rng doesn't work"
        );
    }

    #[test]
    fn test_run_genetic_algorithm() {
        let mut gen_alg = GenAlg::<DummyGenetic>::new(POP_SIZE, None);
        let result = gen_alg.run_genetic_algorithm(NUM_GENS, 0.1, 0.1, 0).unwrap();
        let gen_vec = &gen_alg.current_population;

        println!("{:?}", gen_vec);
        println!("{:?}", result);

        assert_eq!(result.len(), POP_SIZE);
        assert_eq!(gen_alg.current_generation, NUM_GENS);
        assert!(
            result.iter().all(|individual| (0..MAX_RAND).contains(&individual.value)),
            "One or more values are out of range!"
        );
        assert_eq!(&result, gen_vec);

    }
}

