use crate::individual::{dlx, genetic::Genetic};

/// Individual for the DLX algorithm
#[derive(Clone, Debug)]
pub struct Individual {
    instructions: Vec<dlx::Individual>,
}

impl Genetic for Individual {
    fn fitness(&self) -> f32 {
        todo!()
    }

    fn generate() -> Self {
        todo!()
    }

    fn crossover(&self, other: &Self) -> Self {
        todo!()
    }

    fn mutate(&mut self) -> () {
        todo!()
    }
}
