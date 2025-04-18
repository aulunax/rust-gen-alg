use std::error::Error;

use crate::individual::genetic::Genetic;

pub struct GenAlg<T: Genetic + Clone> {
    current_population: Vec<T>,
    current_generation: usize,
}

impl<T: Genetic + Clone> GenAlg<T> {
    pub fn run_genetic_algorithm(
        &self,
        num_of_generations: usize,
    ) -> Result<&Vec<T>, Box<dyn Error>> {

        Ok(&self.current_population)
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

    #[derive(Clone)]
    struct DummyGenetic {
        value: i32,
    }

    impl Genetic for DummyGenetic {
        fn generate() -> Self {
            let mut rng = rand::rng();
            let random_value = rng.random_range(0..100);

            DummyGenetic { value: random_value }
        }

        fn fitness(&self) -> f32 {
            todo!();
        }

    }


    #[test]
    fn test_gen_alg_creation() {
        let population_size = 50;
        
        let gen_alg = GenAlg::<DummyGenetic>::new(population_size, None);
        let gen_vec = &gen_alg.current_population;

        assert_eq!(gen_alg.current_population.len(), population_size);
        assert!(
            gen_vec.iter().all(|individual| (0..100).contains(&individual.value)),
            "One or more values are out of range!"
        );
        assert!(!(gen_vec.iter().all(|x| x.value == gen_vec[0].value)), 
            "All elements in vector are equal, rng doesn't work"
        );
    }

    #[test]
    fn test_gen_alg_creation_init_pop() {
        let population_size = 50;
        let init_pop: Vec<DummyGenetic> = (0..population_size).map(|_| DummyGenetic::generate()).collect();

        let gen_alg = GenAlg::<DummyGenetic>::new(population_size, Some(&init_pop));
        let gen_vec = &gen_alg.current_population;

        assert_eq!(gen_alg.current_population.len(), population_size);
        assert!(
            gen_vec.iter().all(|individual| (0..100).contains(&individual.value)),
            "One or more values are out of range!"
        );
        assert!(!(gen_vec.iter().all(|x| x.value == gen_vec[0].value)), 
            "All elements in vector are equal, rng doesn't work"
        );
    }

    // #[test]
    // fn test_run_genetic_algorithm() {
    //     let population_size = 5;
    //     let gen_alg = GenAlg::<DummyGenetic>::new(population_size, None);
    //     let result = gen_alg.run_genetic_algorithm(10).unwrap();

    //     assert_eq!(result.len(), population_size);
    // }
}

