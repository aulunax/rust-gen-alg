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
        elite_count: usize,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        if elite_count > self.current_population.len() {
            panic!(
                "Number of elite individuals can't be higher than total number of individuals in a generation"
            );
        }

        for _ in 0..num_of_generations {
            //
            // Actual code goes here
            //

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
}

#[cfg(test)]
mod tests {
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
            -(self.a - self.b).abs() as f32
        }

        fn crossover(&self, other: Self) -> Self {
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

    const POP_SIZE: usize = 20;
    const NUM_GENS: usize = 10;

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
        let result = gen_alg
            .run_genetic_algorithm(NUM_GENS, 0.1, 0.1, 0)
            .unwrap();
        let gen_vec = &gen_alg.current_population;

        println!("{:?}", gen_vec);
        println!("{:?}", result);

        assert_eq!(result.len(), POP_SIZE);
        assert_eq!(gen_alg.current_generation, NUM_GENS);
        assert!(
            are_vals_in_range(&result),
            "One or more values are out of range!"
        );
        assert_eq!(&result, gen_vec);
    }
}
